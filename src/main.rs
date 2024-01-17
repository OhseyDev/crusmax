mod web;

use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;

use hyper::body::Bytes;
use hyper::server::conn::{http1, http2};
use hyper::service::service_fn;
use hyper::{Request, Response};

use hyper_util::rt::TokioIo;

use tokio::net::TcpListener;

use log::{info, trace, warn};

fn init_syslog() {
    use syslog::{Facility, Formatter3164, BasicLogger};

    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "crusmax".into(),
        pid: std::process::id()
    };

    match syslog::unix(formatter) {
        Err(_e) => warn!("Unable to connect to syslog"),
        Ok(writer) => {
            let res = log::set_boxed_logger(Box::new(BasicLogger::new(writer)));
            if res.is_err() { warn!("Unable to make syslog program log location"); }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if cfg!(release) {
        init_syslog();
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    
    let mut up = true;

    while up {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            /*if http2.serve_connection(io, web::Service::new()).await.is_ok() {
            } else */if let Err(_e) = http1::Builder::new().serve_connection(io, web::Service::new()).await {
                warn!("Unable to serve client");
            }
        });
    }
    Ok(())
}
