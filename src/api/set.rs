use crate::guards::value::ValueGuard;
use crate::guards::auth::AuthGuard;
use crate::db::connection::Pool;
use crate::db::nonrepeating::*;
use crate::auth::*;

use rocket_contrib::json::JsonValue;
use diesel::prelude::*;
use rocket::State;

fn status_ok(status: &str, conn: &MysqlConnection) -> JsonValue {
    json!({
        "status": status.to_string(),
        "key": generate_auth_key(conn)
    })
}

fn status_err(status: &str) -> JsonValue {
    json!({
        "status": status.to_string()
    })
}

fn set_param(key: AuthKey, db_key: &str, db_value: &str, pool: State<Pool>) -> JsonValue {
    let conn = pool.get().unwrap();

    if !verify_auth_key(key, &conn) {
        status_err("not logged in")
    } else {
        let nr = Nonrepeating::new(&pool);
        match nr.set(db_key, db_value) {
            Ok(_) => status_ok("success", &conn),
            Err(_) => status_err("error")
        }
    }
}

#[get("/setlogin")]
pub fn login(key: AuthGuard, value: ValueGuard, conn: State<Pool>)  -> JsonValue {
    set_param(key.get(), "login", &value.get(), conn)
}

#[get("/setpass")]
pub fn password_hash(key: AuthGuard, value: ValueGuard, conn: State<Pool>) -> JsonValue {
    set_param(key.get(), "pass_hash", &value.get(), conn)
}
