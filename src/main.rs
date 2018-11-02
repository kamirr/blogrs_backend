extern crate hyper;
mod service;

use hyper::service::service_fn_ok;
use hyper::rt::Future;
use hyper::Server;

fn main() {
    let port = 3000;
    let addr = ([127, 0, 0, 1], port).into();

    let new_svc = || {
        service_fn_ok(service::make_service)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
