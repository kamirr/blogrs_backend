use rocket::request::{self, Request, FromRequest};
use rocket::outcome::Outcome::*;
use rocket::http::Status;

pub struct ValueGuard {
    val: String
}

impl ValueGuard {
    pub fn get(&self) -> String {
        self.val.clone()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ValueGuard {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ValueGuard, ()> {
        let res = request
            .headers()
            .get("X-Value")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        match res.len() {
            1 => Success(ValueGuard{ val: res[0].clone() }),
            _ => Failure((Status::Forbidden, ()))
        }
    }
}
