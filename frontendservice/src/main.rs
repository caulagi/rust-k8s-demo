#[macro_use]
extern crate tower_web;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod grpc_client;
mod views;

use crate::views::Quotation;
use tower_web::middleware::log::LogMiddleware;
use tower_web::ServiceBuilder;

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
