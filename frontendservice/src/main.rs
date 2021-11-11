use std::{env, error::Error, net::SocketAddr};

use axum::{handler::get, http::StatusCode, response::IntoResponse, Router};
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};
use tracing_attributes::instrument;

pub mod quotation {
    tonic::include_proto!("quotation");
}

use quotation::{quotation_client::QuotationClient, QuotationRequest};

async fn get_quotation() -> Result<String, Box<dyn std::error::Error>> {
    let service_hostname = env::var("QUOTATION_SERVICE_HOSTNAME")?;
    let mut client = QuotationClient::connect(format!("http://{}:9001", service_hostname)).await?;
    let response = client
        .get_random_quotation(tonic::Request::new(QuotationRequest {}))
        .await?;
    Ok(response.into_inner().message)
}

#[instrument]
async fn handler() -> impl IntoResponse {
    match get_quotation().await {
        Ok(val) => {
            debug!("Received quotation: {:?}", val);
            (StatusCode::OK, val)
        }
        Err(e) => {
            error!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

#[instrument]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    use tracing_subscriber::EnvFilter;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("async_fn=trace".parse()?))
        .init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let app = Router::new()
        .route("/", get(handler))
        .layer(TraceLayer::new_for_http());

    info!("Frontend service starting on {:?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
