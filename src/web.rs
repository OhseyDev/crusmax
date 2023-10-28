use std::future::Future;
use std::pin::Pin;

use hyper::{Response, Request};
use hyper::body::{Bytes, Incoming as IncomingBody};

use hyper::service::Service as HyperService;

use http_body_util::Full;

pub struct Service {}

impl Service {
    pub fn new() -> Self {
        Service {}
    }
    fn mk_response(s: String) -> Result<<Self as HyperService<Request<IncomingBody>>>::Response, <Self as HyperService<Request<IncomingBody>>>::Error> {
        Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
    }
}

impl HyperService<Request<IncomingBody>> for Service {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        let res = match req.uri().path() {
            "/" => Self::mk_response("Index".into()),
            _ => return Box::pin(async { Self::mk_response("Not Found!".into()) })
        };

        Box::pin(async { res })
    }
}
