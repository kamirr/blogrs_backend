use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome::*;
use rocket::http::Status;

use crate::auth_key::*;

pub struct AuthGuard {
    pub key: AuthKey
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthGuard {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AuthGuard, ()> {
        let res = request
            .headers()
            .get("X-Api-Key")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        match res.len() {
            1 => Success(AuthGuard { key: res[0].clone() }),
            _ => Failure((Status::Forbidden, ()))
        }
    }
}
