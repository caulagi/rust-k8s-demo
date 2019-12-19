extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::env;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::process::{Command, Stdio};
use warp::Filter;

pub mod fortune {
    tonic::include_proto!("fortune");
}

use fortune::{fortune_client::FortuneClient, FortuneRequest};

#[derive(Clone, Debug)]
pub struct Quotation;

/// Resolve the hostname (like fortuneservice) to an ip where
/// the service is running. The result is used both as a string
/// and IPV4Addr, so returning string is more generic.
///
/// FIXME: Use a better solution (maybe domain or trust-dns)?
fn hostname_to_ip() -> String {
    let service_hostname = match env::var("FORTUNE_SERVICE_HOSTNAME") {
        Ok(val) => val,
        Err(_) => panic!("Not able to find FORTUNE_SERVICE_HOSTNAME"),
    };
    let getent_path = match env::var("GETENT_PATH") {
        Ok(val) => val,
        Err(_) => panic!("Not able to find GETENT_PATH"),
    };
    info!(
        "resolving fortuneservice: {}, getent_path: {}",
        service_hostname, getent_path
    );
    let child = Command::new(getent_path)
        .args(&["hosts", service_hostname.as_str()])
        .stdout(Stdio::piped())
        .spawn()
        .expect("should resolve");
    debug!("{:?}", child);
    let out = match child.stdout {
        Some(stdout) => stdout,
        None => panic!("getent failed"),
    };
    debug!("{:?}", out);
    let stdio = unsafe { Stdio::from_raw_fd(out.as_raw_fd()) };
    let first_ip = Command::new("head")
        .args(&["-1"])
        .stdin(stdio)
        .output()
        .expect("2");
    let a = String::from_utf8(first_ip.stdout).expect("first ip stdout");
    let mut parts = a.split_whitespace();
    let ip = match parts.next() {
        Some(x) => x,
        None => panic!("No ip"),
    };
    info!("{} resolved to {:?}", service_hostname, ip);
    ip.to_string()
}

async fn get_fortune() -> Result<std::boxed::Box<std::string::String>, Box<dyn std::error::Error>> {
    let uri = format!("http://{}:50051", hostname_to_ip());
    let mut client = FortuneClient::connect(uri).await?;
    let request = tonic::Request::new(FortuneRequest {});
    let response = client.get_random_fortune(request).await?;
    info!("RESPONSE={:?}", response);
    Ok(Box::new(response.into_inner().message.to_string()))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let routes = warp::any().and_then(|| {
        async move {
            let msg = get_fortune().await;
            info!("msg: {:?}", msg);
            match msg {
                Ok(val) => Ok::<String, warp::Rejection>(val.to_string()),
                Err(_) => Ok::<String, warp::Rejection>("error".to_string()),
            }
        }
    });

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
