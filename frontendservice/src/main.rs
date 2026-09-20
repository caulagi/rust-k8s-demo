use std::{env, error::Error, net::SocketAddr, time::Instant};

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use metrics::{counter, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};
use tracing_attributes::instrument;

pub mod quotation {
    // https://github.com/tokio-rs/prost/issues/661
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("quotation");
}

use quotation::{quotation_client::QuotationClient, QuotationRequest};

async fn get_quotation() -> Result<String, Box<dyn std::error::Error>> {
    let service_hostname = env::var("QUOTATION_SERVICE_HOSTNAME")?;
    let mut client = QuotationClient::connect(format!("http://{service_hostname}:9001")).await?;
    let response = client
        .get_random_quotation(tonic::Request::new(QuotationRequest {}))
        .await?;
    Ok(response.into_inner().message)
}

#[instrument]
async fn handler() -> impl IntoResponse {
    let start = Instant::now();
    let (status, body) = match get_quotation().await {
        Ok(val) => {
            debug!("Received quotation: {:?}", val);
            (StatusCode::OK, val)
        }
        Err(e) => {
            error!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    };
    counter!("http_requests_total", "status" => status.as_u16().to_string()).increment(1);
    histogram!("http_request_duration_seconds").record(start.elapsed().as_secs_f64());
    (status, body)
}

#[instrument]
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

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let app = Router::new()
        .route("/", get(handler))
        .layer(TraceLayer::new_for_http());

    info!("Frontend service starting on {:?}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
