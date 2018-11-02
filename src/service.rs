use hyper::{StatusCode, Method, Body, Request, Response};

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

pub fn make_service(req: Request<Body>) -> Response<Body> {
    let uri = req.uri().path().to_string();
    let method = req.method();

    match method {
        &Method::GET => get(uri),
        &Method::PUT => put(uri),
        _ => invalid_request()
    }
}
