use std::env;

use trust_dns_resolver::Resolver;
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
fn hostname_to_ip(name: &str) -> String {
    let resolver = Resolver::from_system_conf().unwrap();
    let response = resolver.lookup_ip(name).unwrap();
    let address = response.iter().next().expect("no addresses returned!");
    log::debug!("{} resolved to {:?}", name, address);
    address.to_string()
}

async fn get_fortune() -> Result<std::boxed::Box<std::string::String>, Box<dyn std::error::Error>> {
    let service_hostname = match env::var("FORTUNE_SERVICE_HOSTNAME") {
        Ok(val) => val,
        Err(_) => panic!("Not able to find FORTUNE_SERVICE_HOSTNAME"),
    };
    let uri = format!("http://{}:50051", hostname_to_ip(&service_hostname));
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
