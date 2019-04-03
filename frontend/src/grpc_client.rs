extern crate futures;
extern crate http;
extern crate prost;
extern crate tokio;
extern crate tower;
extern crate tower_add_origin;
extern crate tower_grpc;
extern crate tower_h2;
extern crate tower_service;

use futures::{Future, Poll};
use tokio::executor::DefaultExecutor;
use tokio::net::tcp::{ConnectFuture, TcpStream};
use tower::MakeService;
use tower_grpc::Request;
use tower_h2::client;
use tower_service::Service;

pub mod fortune {
    include!(concat!(env!("OUT_DIR"), "/fortune.rs"));
}

pub fn make_request() {
    let uri: http::Uri = "http://localhost:50051".parse().unwrap();
    let mut make_client = client::Connect::new(Dst, Default::default(), DefaultExecutor::current());

    let task = make_client
        .make_service(())
        .map(move |conn| {
            use fortune::client::Fortune;

            let conn = tower_add_origin::Builder::new()
                .uri(uri)
                .build(conn)
                .unwrap();

            Fortune::new(conn)
        })
        .and_then(|mut client| {
            use fortune::FortuneRequest;

            client
                .get_random_fortune(Request::new(FortuneRequest {}))
                .map_err(|e| panic!("gRPC request failed; err={:?}", e))
        })
        .map(|response| {
            info!("RESPONSE = {:?}", response);
        })
        .map_err(|e| {
            error!("ERR = {:?}", e);
        });
    tokio::spawn(task);
}

struct Dst;

impl Service<()> for Dst {
    type Response = TcpStream;
    type Error = ::std::io::Error;
    type Future = ConnectFuture;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(().into())
    }

    fn call(&mut self, _: ()) -> Self::Future {
        TcpStream::connect(&([127, 0, 0, 1], 50051).into())
    }
}
