use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome::*;
use rocket::http::Status;

pub struct LoginGuard {
    pub hash: String
}

impl<'a, 'r> FromRequest<'a, 'r> for LoginGuard {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<LoginGuard, ()> {
        let res = request
            .headers()
            .get("X-Login-Hash")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        match res.len() {
            1 => Success(LoginGuard { hash: res[0].clone() }),
            _ => Failure((Status::Forbidden, ()))
        }
    }
}
