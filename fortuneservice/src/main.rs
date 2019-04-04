extern crate futures;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate prost;
extern crate tokio;
extern crate tower_grpc;
extern crate tower_h2;

use fortune::{server, FortuneRequest, FortuneResponse};

use futures::{future, Future, Stream};
use std::process::Command;
use tokio::executor::DefaultExecutor;
use tokio::net::TcpListener;
use tower_grpc::{Request, Response};
use tower_h2::Server;

pub mod fortune {
    include!(concat!(env!("OUT_DIR"), "/fortune.rs"));
}

#[derive(Clone, Debug)]
struct Service;

impl server::Fortune for Service {
    type GetRandomFortuneFuture =
        future::FutureResult<Response<FortuneResponse>, tower_grpc::Status>;

    fn get_random_fortune(
        &mut self,
        request: Request<FortuneRequest>,
    ) -> Self::GetRandomFortuneFuture {
        info!("REQUEST = {:?}", request);
        let cookie = Command::new(env!("FORTUNE_PATH"))
            .output()
            .expect("failed to execute process");
        let response = Response::new(FortuneResponse {
            message: String::from_utf8_lossy(&cookie.stdout).to_string(),
        });

        future::ok(response)
    }
}

pub fn main() {
    pretty_env_logger::init();

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
