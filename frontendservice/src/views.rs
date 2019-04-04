extern crate futures;

use crate::grpc_client::get_fortune;
use futures::prelude::Future;

#[derive(Clone, Debug)]
pub struct Quotation;

impl_web! {
    impl Quotation {
        #[get("/")]
        fn index(&self) -> impl Future<Item = String, Error = ()> {
            get_fortune()
        }
    }
}
