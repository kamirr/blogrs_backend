use crate::guards::value::ValueGuard;
use crate::guards::auth::AuthGuard;
use crate::db::connection::Pool;
use crate::db::nonrepeating::*;
use crate::auth::*;

use rocket_contrib::json::JsonValue;
use rocket::State;

fn status_ok(status: &str, pool: &State<Pool>) -> JsonValue {
    json!({
        "status": status.to_string(),
        "key": generate_auth_key(&pool)
    })
}

fn status_err(status: &str) -> JsonValue {
    json!({
        "status": status.to_string()
    })
}

fn set_param(key: AuthKey, db_key: &str, db_value: &str, pool: State<Pool>) -> JsonValue {
    if !verify_auth_key(key, &pool) {
        status_err("not logged in")
    } else {
        let nr = Nonrepeating::new(&pool);
        match nr.set(db_key, db_value) {
            Ok(_) => status_ok("success", &pool),
            Err(_) => status_err("error")
        }
    }
}

#[get("/setlogin")]
pub fn login(key: AuthGuard, value: ValueGuard, pool: State<Pool>)  -> JsonValue {
    set_param(key.get(), "login", &value.get(), pool)
}

#[get("/setpass")]
pub fn password_hash(key: AuthGuard, value: ValueGuard, pool: State<Pool>) -> JsonValue {
    set_param(key.get(), "pass_hash", &value.get(), pool)
}
