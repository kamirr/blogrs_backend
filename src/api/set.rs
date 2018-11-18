use crate::guards::value::ValueGuard;
use crate::guards::auth::AuthGuard;
use crate::db::connection::Pool;
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

fn save_value_to_db(db_key: &str, db_value: &str, conn: &MysqlConnection) -> JsonValue {
    use crate::db::schema::nonrepeating::dsl::*;

    let res = diesel::update(nonrepeating.find(db_key))
        .set(
            value.eq(db_value)
        )
        .execute(conn);

    match res {
        Ok(_) => status_ok("success", conn),
        Err(_) => status_err("DB error")
    }
}

fn set_param(key: AuthKey, db_key: &str, db_value: &str, conn: State<Pool>) -> JsonValue {
    let conn = conn.get().unwrap();

    if !verify_auth_key(key, &conn) {
        status_err("not logged in")
    } else {
        save_value_to_db(db_key, db_value, &conn)
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
