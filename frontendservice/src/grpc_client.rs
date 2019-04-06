extern crate futures;
extern crate http;
extern crate prost;
extern crate tokio;
extern crate tower;
extern crate tower_add_origin;
extern crate tower_buffer;
extern crate tower_grpc;
extern crate tower_h2;
extern crate tower_service;

use futures::{Future, Poll};
use tokio::executor::DefaultExecutor;
use tokio::net::tcp::{ConnectFuture, TcpStream};
use tokio::prelude::*;
use tower::MakeService;
use tower_add_origin::{AddOrigin, Builder};
use tower_buffer::Buffer;
use tower_grpc::{BoxBody, Request};
use tower_h2::client::{Connect, ConnectError, Connection};
use tower_service::Service;

pub mod fortune {
    include!(concat!(env!("OUT_DIR"), "/fortune.rs"));
}

type Buf =
    Buffer<AddOrigin<Connection<TcpStream, DefaultExecutor, BoxBody>>, http::Request<BoxBody>>;
type RpcClient = fortune::client::Fortune<Buf>;

fn create_client(
) -> impl Future<Item = RpcClient, Error = ConnectError<std::io::Error>> + Send + 'static {
    let uri: http::Uri = format!("http://{}", env!("FORTUNE_SERVICE_HOSTNAME"))
        .parse()
        .unwrap();
    let mut make_client = Connect::new(Dst, Default::default(), DefaultExecutor::current());

    make_client.make_service(()).map(|c| {
        let uri = uri;
        let connection = Builder::new().uri(uri).build(c).unwrap();

        let buffer = match Buffer::new(connection, 128) {
            Ok(b) => b,
            _ => panic!("Unable to accept connection"),
        };

        fortune::client::Fortune::new(buffer)
    })
}

pub fn get_fortune() -> impl Future<Item = String, Error = ()> {
    create_client()
        .map_err(|e| {
            error!("error = {:?}", e);
        })
        .and_then(|mut client| {
            use fortune::FortuneRequest;

            client
                .get_random_fortune(Request::new(FortuneRequest {}))
                .map_err(|e| panic!("gRPC request failed; err={:?}", e))
        })
        .then(|r| {
            let resp = match r {
                Ok(b) => b,
                _ => panic!("Unknown response"),
            };
            future::ok(resp.into_inner().message.to_string())
        })
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
        use std::net::SocketAddr;
        use std::os::unix::io::AsRawFd;
        use std::os::unix::io::FromRawFd;
        use std::process::{Command, Stdio};

        info!("resolving fortuneservice");
        let child = Command::new(env!("GETENT_PATH"))
            .args(&["hosts", env!("FORTUNE_SERVICE_HOSTNAME")])
            .stdout(Stdio::piped())
            .spawn()
            .expect("should resolve");
        info!("{:?}", child);
        let out = match child.stdout {
            Some(stdout) => stdout,
            None => panic!("getent failed"),
        };
        info!("{:?}", out);
        let stdio = unsafe { Stdio::from_raw_fd(out.as_raw_fd()) };
        let first_ip = Command::new("head")
            .args(&["-1"])
            .stdin(stdio)
            .output()
            .expect("2");
        let a = String::from_utf8(first_ip.stdout).expect("first ip stdout");
        let mut parts = a.split_whitespace();
        let ip = match parts.next() {
            Some(x) => x,
            None => panic!("No ip"),
        };
        info!("**** {:?}", ip);

        let addr = format!("{}:50051", ip).parse().unwrap();
        info!("*** connecting to {}", addr);
        TcpStream::connect(&addr)
    }
}
