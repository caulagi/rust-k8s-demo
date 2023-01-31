use std::{env, error::Error, time::Duration};

use tokio_postgres::NoTls;
use tonic::{transport::Server, Request, Response, Status};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{debug, info};

pub mod quotation {
    // https://github.com/tokio-rs/prost/issues/661
    #![allow(clippy::derive_partial_eq_without_eq)]
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
        debug!("REQUEST = {:?}", request);
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
                eprintln!("connection error: {e}");
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
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    use tracing_subscriber::EnvFilter;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("async_fn=trace".parse()?))
        .init();

    let addr = "0.0.0.0:9001".parse().unwrap();
    let quotationr = MyQuotation::default();

    // Build our middleware stack
    let layer = ServiceBuilder::new()
        // Set a timeout
        .timeout(Duration::from_secs(10))
        // Log all requests and responses
        .layer(
            TraceLayer::new_for_grpc().make_span_with(DefaultMakeSpan::new().include_headers(true)),
        )
        .into_inner();

    info!("Quotation service starting on {:?}", addr);
    Server::builder()
        .layer(layer)
        .add_service(QuotationServer::new(quotationr))
        .serve(addr)
        .await?;

    Ok(())
}
