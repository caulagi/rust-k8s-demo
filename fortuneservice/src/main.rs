use std::process::Command;

use tonic::{transport::Server, Request, Response, Status};

pub mod fortune {
    tonic::include_proto!("fortune");
}

use fortune::{
    fortune_server::{Fortune, FortuneServer},
    FortuneRequest, FortuneResponse,
};

#[derive(Default)]
pub struct MyFortune {}

#[tonic::async_trait]
impl Fortune for MyFortune {
    async fn get_random_fortune(
        &self,
        request: Request<FortuneRequest>,
    ) -> Result<Response<FortuneResponse>, Status> {
        log::info!("REQUEST = {:?}", request);
        let cookie = Command::new(env!("FORTUNE_PATH"))
            .output()
            .expect("failed to execute process");
        let response = fortune::FortuneResponse {
            message: String::from_utf8_lossy(&cookie.stdout).to_string(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let addr = "0.0.0.0:50051".parse().unwrap();
    let fortuner = MyFortune::default();

    log::info!("Fortune service starting on {:?}", addr);
    Server::builder()
        .add_service(FortuneServer::new(fortuner))
        .serve(addr)
        .await?;

    Ok(())
}
