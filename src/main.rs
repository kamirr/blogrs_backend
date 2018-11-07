#![feature(label_break_value)]

extern crate hyper;

mod dynamic_content;
mod static_content;
mod content;
mod service;

use hyper::service::service_fn_ok;
use crate::content::Content;
use hyper::rt::Future;
use hyper::Server;

fn run_server(cont: Content) {
    let port = 3000;
    let addr = ([127, 0, 0, 1], port).into();

    let new_svc = move || {
        let cont = cont.clone();
        service_fn_ok(move |req| {
            service::serve(req, cont.clone())
        })
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}

fn main() {
    run_server(Content::new("frontend/"));
}
