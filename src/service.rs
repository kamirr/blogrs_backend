use hyper::{StatusCode, Method, Body, Request, Response};
use crate::content::Content;

fn invalid_request(code: u16) -> Response<Body> {
    Response::builder()
        .status(StatusCode::from_u16(code).unwrap())
        .body(Body::empty())
        .unwrap()
}

fn get(uri: String, mut cont: Content) -> Response<Body> {
    let res = if uri.starts_with("/dyn/") {
        cont.dynamic_c.fetch(uri)
    } else {
        cont.static_c.fetch(uri)
    };

    match res {
        Some(s) => Response::new(Body::from(s)),
        None => invalid_request(404)
    }
}

fn put(_: String) -> Response<Body> {
    Response::new(Body::empty())
}

pub fn serve(req: Request<Body>, cont: Content) -> Response<Body> {
    let uri = req.uri().path().to_string();
    let method = req.method();

    match method {
        &Method::GET => get(uri, cont),
        &Method::PUT => put(uri),
        _ => invalid_request(422)
    }
}
