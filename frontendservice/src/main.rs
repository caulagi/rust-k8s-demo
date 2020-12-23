use std::{env, io, net::SocketAddr};

use tokio::net;
use tokio_compat_02::FutureExt;
use warp::Filter;
pub mod quotation {
    tonic::include_proto!("quotation");
}

use quotation::{quotation_client::QuotationClient, QuotationRequest};

#[derive(Debug)]
struct ServerError;

impl warp::reject::Reject for ServerError {}

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

async fn get_quotation() -> Result<Box<String>, Box<dyn std::error::Error>> {
    let service_hostname = match env::var("QUOTATION_SERVICE_HOSTNAME") {
        Ok(val) => format!("{}:50051", val),
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
    log::trace!("RESPONSE={:?}", response);
    Ok(Box::new(response.into_inner().message.to_string()))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = warp::any().and_then(|| async move {
        match get_quotation().await {
            Ok(val) => {
                log::debug!("Got random quotation: {:?}", val);
                Ok::<String, warp::Rejection>(val.to_string())
            }
            Err(e) => {
                log::error!("ERROR: {:?}", e);
                Err(warp::reject::custom(ServerError))
            }
        }
    });

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    log::info!("Frontend service starting on {:?}", addr);
    warp::serve(routes).run(addr).compat().await;
}
