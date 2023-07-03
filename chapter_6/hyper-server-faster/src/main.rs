extern crate futures;
extern crate hyper;
extern crate net2;
extern crate num_cpus;
extern crate tokio_core;
use futures::{future::FutureResult, Stream};
use hyper::{
    header::ContentLength,
    server::{Http, Request, Response, Service},
    Error, Get, StatusCode,
};
use net2::{unix::UnixTcpBuilderExt, TcpBuilder};
use std::{net::SocketAddr, sync::Arc, thread, time::Duration};
use tokio_core::{net::TcpListener, reactor::Core};

fn heavy_work() -> String {
    let duration = Duration::from_millis(200);
    thread::sleep(duration);
    "done".to_string()
}

#[derive(Clone, Copy)]
struct Echo;

impl Service for Echo {
    type Error = Error;
    type Request = Request;
    type Response = Response;
    type Future = FutureResult<Response, Error>;

    fn call(&self, req: Request) -> Self::Future {
        futures::future::ok(match (req.method(), req.path()) {
            (&Get, "/data") => {
                let body = heavy_work().into_bytes();
                Response::new()
                    .with_header(ContentLength(body.len() as u64))
                    .with_body(body)
            }
            _ => Response::new().with_status(StatusCode::NotFound),
        })
    }
}

// One server instance
fn serve(addr: &SocketAddr, protocol: &Http) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let listener = TcpBuilder::new_v4()
        .unwrap()
        .reuse_port(true)
        .unwrap()
        .bind(addr)
        .unwrap()
        .listen(128)
        .unwrap();
    let listener = TcpListener::from_listener(listener, addr, &handle).unwrap();

    core.run(listener.incoming().for_each(|(socket, addr)| {
        protocol.bind_connection(&handle, socket, addr, Echo);
        Ok(())
    }))
    .unwrap();
}

// Starts num number of serving threads
fn start_server(num: usize, addr: &str) {
    let addr = addr.parse().unwrap();
    let protocol = Arc::new(Http::new());
    {
        for _ in 0..num - 1 {
            let protocol = Arc::clone(&protocol);
            thread::spawn(move || serve(&addr, &protocol));
        }
    }
    serve(&addr, &protocol);
}

fn main() {
    start_server(num_cpus::get(), "0.0.0.0:3000");
}
