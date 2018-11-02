#![feature(label_break_value)]

extern crate hyper;

mod static_content;
mod service;

use crate::static_content::StaticContent;
use hyper::service::service_fn_ok;
use hyper::rt::Future;
use hyper::Server;

fn run_server(sc: StaticContent) {
    let port = 3000;
    let addr = ([127, 0, 0, 1], port).into();

    let new_svc = move || {
        let sc = sc.clone();
        service_fn_ok(move |req| {
            service::serve(req, sc.clone())
        })
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}

fn main() {
    run_server(StaticContent::new("frontend/"));
}
