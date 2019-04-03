#[macro_use]
extern crate tower_web;
extern crate pretty_env_logger;
extern crate tokio;
#[macro_use]
extern crate log;
mod grpc_client;

use grpc_client::make_request;
use tower_web::middleware::log::LogMiddleware;
use tower_web::ServiceBuilder;

#[derive(Clone, Debug)]
struct Quotation;

impl_web! {
    impl Quotation {
        #[get("/")]
        fn get_fortune(&self) -> Result<&'static str, ()> {
            make_request();
            Ok("whee!")
        }
    }
}

pub fn main() {
    pretty_env_logger::init();
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    info!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(Quotation)
        .middleware(LogMiddleware::new("frontend"))
        .run(&addr)
        .unwrap();
}
