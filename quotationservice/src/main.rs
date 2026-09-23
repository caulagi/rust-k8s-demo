use std::{env, error::Error, net::SocketAddr, time::Instant};

use metrics::{counter, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use redis::{aio::ConnectionManager, AsyncCommands};
use tokio::sync::OnceCell;
use tokio_postgres::NoTls;
use tonic::{transport::Server, Request, Response, Status};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{debug, info, warn};

pub mod quotation {
    tonic::include_proto!("quotation");
}

use quotation::{
    quotation_server::{Quotation, QuotationServer},
    QuotationRequest,
    QuotationResponse,
};

const QUOTATION_COUNT: i64 = 36937;
const CACHE_TTL_SECONDS: u64 = 300;

/// A cache in front of Postgres. The connection is made on first use and
/// re-made after a failure, so Redis may come up after this service and may
/// go away without taking quotations with it.
pub struct Cache {
    client: redis::Client,
    connection: OnceCell<ConnectionManager>,
}

impl Cache {
    fn from_env() -> Result<Option<Self>, redis::RedisError> {
        let Ok(host) = env::var("REDIS_SERVICE") else {
            info!("REDIS_SERVICE is not set, serving quotations without a cache");
            return Ok(None);
        };
        let client = redis::Client::open(format!("redis://{host}:6379/"))?;
        Ok(Some(Self {
            client,
            connection: OnceCell::new(),
        }))
    }

    async fn connection(&self) -> Result<ConnectionManager, redis::RedisError> {
        self.connection
            .get_or_try_init(|| ConnectionManager::new(self.client.clone()))
            .await
            .cloned()
    }

    async fn get(&self, key: &str) -> Result<Option<String>, redis::RedisError> {
        self.connection().await?.get(key).await
    }

    async fn set(&self, key: &str, value: &str) -> Result<(), redis::RedisError> {
        self.connection()
            .await?
            .set_ex::<_, _, ()>(key, value, CACHE_TTL_SECONDS)
            .await
    }
}

#[derive(Default)]
pub struct MyQuotation {
    cache: Option<Cache>,
}

impl MyQuotation {
    async fn cached(&self, key: &str) -> Option<String> {
        let cache = self.cache.as_ref()?;
        match cache.get(key).await {
            Ok(Some(value)) => {
                counter!("cache_requests_total", "result" => "hit").increment(1);
                Some(value)
            }
            Ok(None) => {
                counter!("cache_requests_total", "result" => "miss").increment(1);
                None
            }
            Err(e) => {
                warn!("cache lookup failed: {e}");
                counter!("cache_requests_total", "result" => "error").increment(1);
                None
            }
        }
    }

    async fn remember(&self, key: &str, value: &str) {
        let Some(cache) = self.cache.as_ref() else {
            return;
        };
        if let Err(e) = cache.set(key, value).await {
            warn!("cache store failed: {e}");
        }
    }

    async fn query_postgres(&self, offset: i64) -> Result<String, Status> {
        let start = Instant::now();
        let connect_params = format!(
            "host={} user=postgres password={}",
            env::var("POSTGRES_SERVICE").unwrap(),
            env::var("POSTGRES_PASSWORD").unwrap()
        );
        // Connect to the database.
        let (client, connection) = tokio_postgres::connect(connect_params.as_str(), NoTls)
            .await
            .unwrap();

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {e}");
            }
        });

        let rows = client
            .query(
                "SELECT content, author FROM quotation OFFSET $1 LIMIT 1;",
                &[&offset],
            )
            .await
            .unwrap();
        histogram!("postgres_query_duration_seconds").record(start.elapsed().as_secs_f64());

        let value: &str = rows[0].get(0);
        Ok(value.to_string())
    }
}

#[tonic::async_trait]
impl Quotation for MyQuotation {
    async fn get_random_quotation(
        &self,
        request: Request<QuotationRequest>,
    ) -> Result<Response<QuotationResponse>, Status> {
        let start = Instant::now();
        debug!("REQUEST = {:?}", request);

        let offset = rand::random_range(0..QUOTATION_COUNT);
        let key = format!("quotation:{offset}");
        let message = match self.cached(&key).await {
            Some(value) => value,
            None => {
                let value = self.query_postgres(offset).await?;
                self.remember(&key, &value).await;
                value
            }
        };

        counter!("grpc_requests_total", "method" => "get_random_quotation").increment(1);
        histogram!("grpc_request_duration_seconds", "method" => "get_random_quotation")
            .record(start.elapsed().as_secs_f64());

        Ok(Response::new(QuotationResponse { message }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    use tracing_subscriber::EnvFilter;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("async_fn=trace".parse()?))
        .init();

    let metrics_addr = SocketAddr::from(([0, 0, 0, 0], 9090));
    PrometheusBuilder::new()
        .with_http_listener(metrics_addr)
        .set_buckets(&[
            0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
        ])?
        .install()?;
    info!("Metrics available on {:?}/metrics", metrics_addr);

    let addr = "0.0.0.0:9001".parse().unwrap();
    let quotationr = MyQuotation {
        cache: Cache::from_env()?,
    };

    // Build our middleware stack
    let layer = ServiceBuilder::new()
        // Log all requests and responses
        .layer(
            TraceLayer::new_for_grpc().make_span_with(DefaultMakeSpan::new().include_headers(true)),
        )
        .into_inner();

    info!("Quotation service starting on {:?}", addr);
    Server::builder()
        .layer(layer)
        .add_service(QuotationServer::new(quotationr))
        .serve(addr)
        .await?;

    Ok(())
}
