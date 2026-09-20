use std::{env, error::Error, net::SocketAddr, time::Instant};

use metrics::{counter, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use tokio_postgres::NoTls;
use tonic::{transport::Server, Request, Response, Status};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{debug, info};

pub mod quotation {
    // https://github.com/tokio-rs/prost/issues/661
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("quotation");
}

use quotation::{
    quotation_server::{Quotation, QuotationServer},
    QuotationRequest,
    QuotationResponse,
};

#[derive(Default)]
pub struct MyQuotation {}

#[tonic::async_trait]
impl Quotation for MyQuotation {
    async fn get_random_quotation(
        &self,
        request: Request<QuotationRequest>,
    ) -> Result<Response<QuotationResponse>, Status> {
        let start = Instant::now();
        debug!("REQUEST = {:?}", request);
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
                "SELECT content, author FROM quotation OFFSET floor(random() * 36937) LIMIT 1;",
                &[],
            )
            .await
            .unwrap();

        let value: &str = rows[0].get(0);
        let response = quotation::QuotationResponse {
            message: value.to_string(),
        };
        counter!("grpc_requests_total", "method" => "get_random_quotation").increment(1);
        histogram!("grpc_request_duration_seconds", "method" => "get_random_quotation")
            .record(start.elapsed().as_secs_f64());

        Ok(Response::new(response))
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
    let quotationr = MyQuotation::default();

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
