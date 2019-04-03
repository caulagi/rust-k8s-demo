extern crate bytes;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate log;
extern crate prost;
extern crate tokio;
extern crate tower_grpc;
extern crate tower_h2;

pub mod fortune {
    include!(concat!(env!("OUT_DIR"), "/fortune.rs"));
}

use fortune::{server, FortuneRequest, FortuneResponse};

use futures::{future, Future, Stream};
use tokio::executor::DefaultExecutor;
use tokio::net::TcpListener;
use tower_grpc::{Request, Response};
use tower_h2::Server;

#[derive(Clone, Debug)]
struct Service;

impl server::Fortune for Service {
    type GetRandomFortuneFuture =
        future::FutureResult<Response<FortuneResponse>, tower_grpc::Status>;

    fn get_random_fortune(
        &mut self,
        request: Request<FortuneRequest>,
    ) -> Self::GetRandomFortuneFuture {
        println!("REQUEST = {:?}", request);

        let response = Response::new(FortuneResponse {
            message: "Zomg, it works!".to_string(),
        });

        future::ok(response)
    }
}

pub fn main() {
    env_logger::init();

    let mut h2 = Server::new(
        server::FortuneServer::new(Service),
        Default::default(),
        DefaultExecutor::current(),
    );

    let addr = "127.0.0.1:50051".parse().unwrap();
    let bind = TcpListener::bind(&addr).expect("bind");

    let serve = bind
        .incoming()
        .for_each(move |sock| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            let serve = h2.serve(sock);
            tokio::spawn(serve.map_err(|e| error!("h2 error: {:?}", e)));

            Ok(())
        })
        .map_err(|e| eprintln!("accept error: {}", e));

    tokio::run(serve)
}
