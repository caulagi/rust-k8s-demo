use std::env;

use tokio_postgres::NoTls;
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
        log::debug!("REQUEST = {:?}", request);
        let connect_params = format!(
            "host={} user=postgres password={}",
            env::var("POSTGRES_SERVICE").unwrap(),
            env::var("POSTGRES_PASSWORD").unwrap()
        );
        // Connect to the database.
        let (client, connection) = tokio_postgres::connect(connect_params.as_str(), NoTls)
            .await
            .unwrap();

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let rows = client
            .query(
                "SELECT content, author FROM quotation OFFSET floor(random() * 36937) LIMIT 1;",
                &[],
            )
            .await
            .unwrap();

        let value: &str = rows[0].get(0);
        let response = fortune::FortuneResponse {
            message: value.to_string(),
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
