#[macro_use]
extern crate tower_web;
extern crate tokio;
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
    ::env_logger::init();
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(Quotation)
        .middleware(LogMiddleware::new("web::frontend"))
        .run(&addr)
        .unwrap();
}
