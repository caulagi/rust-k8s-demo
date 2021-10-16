use std::{env, io, net::SocketAddr};

use axum::{handler::get, http::StatusCode, response::IntoResponse, Router};
use tokio::net;
pub mod quotation {
    tonic::include_proto!("quotation");
}

use quotation::{quotation_client::QuotationClient, QuotationRequest};

/// Resolve the hostname (like quotationservice) to an ip where
/// the service is running.
async fn hostname_to_ip(name: &str) -> io::Result<String> {
    for addr in net::lookup_host(name).await? {
        if addr.is_ipv4() {
            log::debug!("{} resolved to {:?}", name, addr);
            return Ok(addr.to_string());
        }
    }

    panic!("Not able to lookup name")
}

async fn get_quotation() -> Result<String, Box<dyn std::error::Error>> {
    let service_hostname = match env::var("QUOTATION_SERVICE_HOSTNAME") {
        Ok(val) => format!("{}:9001", val),
        Err(_) => panic!("Not able to find QUOTATION_SERVICE_HOSTNAME"),
    };
    let address = hostname_to_ip(&service_hostname).await;
    let uri = match address {
        Ok(val) => format!("http://{}", val),
        Err(_) => panic!("Unable to resolve quotation service"),
    };
    let mut client = QuotationClient::connect(uri).await?;
    let request = tonic::Request::new(QuotationRequest {});
    let response = client.get_random_quotation(request).await?;
    Ok(response.into_inner().message)
}

async fn handler() -> impl IntoResponse {
    match get_quotation().await {
        Ok(val) => {
            log::debug!("Received quotation: {:?}", val);
            (StatusCode::OK, val)
        }
        Err(e) => {
            log::error!("ERROR: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let app = Router::new().route("/", get(handler));

    log::info!("Frontend service starting on {:?}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
