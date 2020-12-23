use std::env;

use tokio_compat_02::FutureExt;
use tokio_postgres::NoTls;
use tonic::{transport::Server, Request, Response, Status};

pub mod quotation {
    tonic::include_proto!("quotation");
}

use quotation::{
    quotation_server::{Quotation, QuotationServer},
    QuotationRequest,
    QuotationResponse,
};

#[derive(Default)]
pub struct MyQuotation {}

#[tonic::async_trait]
impl Quotation for MyQuotation {
    async fn get_random_quotation(
        &self,
        request: Request<QuotationRequest>,
    ) -> Result<Response<QuotationResponse>, Status> {
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
        let response = quotation::QuotationResponse {
            message: value.to_string(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let addr = "0.0.0.0:50051".parse().unwrap();
    let quotationr = MyQuotation::default();

    log::info!("Quotation service starting on {:?}", addr);
    Server::builder()
        .add_service(QuotationServer::new(quotationr))
        .serve(addr)
        .compat()
        .await?;

    Ok(())
}
