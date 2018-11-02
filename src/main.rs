extern crate hyper;

use hyper::{StatusCode, Method, Body, Request, Response, Server};
use hyper::service::service_fn_ok;
use hyper::rt::Future;

fn invalid_request() -> Response<Body> {
    Response::builder()
        .status(StatusCode::from_u16(422).unwrap())
        .body(Body::empty())
        .unwrap()
}

fn get(uri: String) -> Response<Body> {
    Response::new(Body::from(uri))
}

fn put(_: String) -> Response<Body> {
    Response::new(Body::empty())
}

fn server(req: Request<Body>) -> Response<Body> {
    let uri = req.uri().path().to_string();
    let method = req.method();

    match method {
        &Method::GET => get(uri),
        &Method::PUT => put(uri),
        _ => invalid_request()
    }
}

fn main() {
    let port = 3000;
    let addr = ([127, 0, 0, 1], port).into();

    let new_svc = || {
        service_fn_ok(server)
    };

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
