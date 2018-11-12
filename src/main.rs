#![feature(label_break_value)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate hyper;

mod dynamic_content;
mod static_content;
mod connection;
mod content;
mod service;
mod schema;
mod models;

use self::connection::establish_sql_connection;
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
    let mut _connection = establish_sql_connection();
    run_server(Content::new("frontend/"));
}
