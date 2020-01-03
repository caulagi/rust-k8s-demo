use std::env;
use std::io;

use tokio::net;
use warp::Filter;
pub mod fortune {
    tonic::include_proto!("fortune");
}

use fortune::{fortune_client::FortuneClient, FortuneRequest};

#[derive(Debug)]
struct ServerError;

impl warp::reject::Reject for ServerError {}

/// Resolve the hostname (like fortuneservice) to an ip where
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

async fn get_fortune() -> Result<Box<String>, Box<dyn std::error::Error>> {
    let service_hostname = match env::var("FORTUNE_SERVICE_HOSTNAME") {
        Ok(val) => format!("{}:50051", val),
        Err(_) => panic!("Not able to find FORTUNE_SERVICE_HOSTNAME"),
    };
    let address = hostname_to_ip(&service_hostname).await;
    let uri = match address {
        Ok(val) => format!("http://{}", val),
        Err(_) => panic!("Unable to resolve fortune service"),
    };
    let mut client = FortuneClient::connect(uri).await?;
    let request = tonic::Request::new(FortuneRequest {});
    let response = client.get_random_fortune(request).await?;
    log::debug!("RESPONSE={:?}", response);
    Ok(Box::new(response.into_inner().message.to_string()))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = warp::any().and_then(|| {
        async move {
            match get_fortune().await {
                Ok(val) => {
                    log::debug!("Got random fortune: {:?}", val);
                    Ok::<String, warp::Rejection>(val.to_string())
                }
                Err(e) => {
                    log::error!("ERROR: {:?}", e);
                    Err(warp::reject::custom(ServerError))
                }
            }
        }
    });

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
