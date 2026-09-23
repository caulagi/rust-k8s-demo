use std::{
    env,
    error::Error,
    net::SocketAddr,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod, Runtime};
use metrics::{counter, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use redis::{aio::ConnectionManager, AsyncCommands};
use rustls::pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer};
use tokio::sync::OnceCell;
use tokio_postgres_rustls::MakeRustlsConnect;
use tonic::{transport::Server, Code, Request, Response, Status};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{debug, error, info, warn};

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
const POSTGRES_POOL_SIZE: usize = 8;
const PROCESS_METRICS_INTERVAL: Duration = Duration::from_secs(15);

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
        let start = Instant::now();
        let result: Result<Option<String>, redis::RedisError> =
            async { self.connection().await?.get(key).await }.await;
        histogram!("cache_request_duration_seconds", "op" => "get")
            .record(start.elapsed().as_secs_f64());
        result
    }

    async fn set(&self, key: &str, value: &str) -> Result<(), redis::RedisError> {
        let start = Instant::now();
        let result: Result<(), redis::RedisError> = async {
            self.connection()
                .await?
                .set_ex::<_, _, ()>(key, value, CACHE_TTL_SECONDS)
                .await
        }
        .await;
        histogram!("cache_request_duration_seconds", "op" => "set")
            .record(start.elapsed().as_secs_f64());
        result
    }
}

/// `dir` holds `ca.crt`, `tls.crt` and `tls.key`, as a cert-manager secret is
/// laid out. The certificate's common name is the database user.
fn postgres_tls(dir: &Path) -> Result<MakeRustlsConnect, Box<dyn Error + Send + Sync>> {
    let mut roots = rustls::RootCertStore::empty();
    for cert in CertificateDer::pem_file_iter(dir.join("ca.crt"))? {
        roots.add(cert?)?;
    }
    let certs = CertificateDer::pem_file_iter(dir.join("tls.crt"))?.collect::<Result<_, _>>()?;
    let key = PrivateKeyDer::from_pem_file(dir.join("tls.key"))?;
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let config = rustls::ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .with_root_certificates(roots)
        .with_client_auth_cert(certs, key)?;
    Ok(MakeRustlsConnect::new(config))
}

fn postgres_pool(host: &str, tls: MakeRustlsConnect) -> Result<Pool, Box<dyn Error + Send + Sync>> {
    let config: tokio_postgres::Config =
        format!("host={host} user=postgres sslmode=require").parse()?;
    let manager = Manager::from_config(
        config,
        tls,
        ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        },
    );
    Ok(Pool::builder(manager)
        .max_size(POSTGRES_POOL_SIZE)
        .runtime(Runtime::Tokio1)
        .build()?)
}

pub struct MyQuotation {
    cache: Option<Cache>,
    postgres: Pool,
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
        let client = self.postgres.get().await.map_err(|e| {
            error!("no database connection: {e}");
            Status::unavailable("database unavailable")
        })?;
        histogram!("postgres_checkout_duration_seconds").record(start.elapsed().as_secs_f64());

        let start = Instant::now();
        let rows = client
            .query(
                "SELECT content, author FROM quotation OFFSET $1 LIMIT 1;",
                &[&offset],
            )
            .await
            .map_err(|e| {
                error!("query failed: {e}");
                Status::internal("database query failed")
            })?;
        histogram!("postgres_query_duration_seconds").record(start.elapsed().as_secs_f64());

        let row = rows
            .first()
            .ok_or_else(|| Status::internal(format!("no quotation at offset {offset}")))?;
        Ok(row.get(0))
    }

    async fn random_quotation(&self) -> Result<String, Status> {
        let offset = rand::random_range(0..QUOTATION_COUNT);
        let key = format!("quotation:{offset}");
        if let Some(value) = self.cached(&key).await {
            return Ok(value);
        }
        let value = self.query_postgres(offset).await?;
        self.remember(&key, &value).await;
        Ok(value)
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

        let result = self.random_quotation().await;

        let code = result.as_ref().map_or_else(Status::code, |_| Code::Ok);
        counter!("grpc_requests_total", "method" => "get_random_quotation", "code" => format!("{code:?}"))
            .increment(1);
        histogram!("grpc_request_duration_seconds", "method" => "get_random_quotation")
            .record(start.elapsed().as_secs_f64());

        result.map(|message| Response::new(QuotationResponse { message }))
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

    let process = metrics_process::Collector::default();
    process.describe();
    tokio::spawn(async move {
        loop {
            process.collect();
            tokio::time::sleep(PROCESS_METRICS_INTERVAL).await;
        }
    });

    let addr = "0.0.0.0:9001".parse().unwrap();
    let tls = postgres_tls(Path::new(&env::var("POSTGRES_TLS_DIR")?))?;
    let quotationr = MyQuotation {
        cache: Cache::from_env()?,
        postgres: postgres_pool(&env::var("POSTGRES_SERVICE")?, tls)?,
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
