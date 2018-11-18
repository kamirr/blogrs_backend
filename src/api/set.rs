use crate::guards::value::ValueGuard;
use crate::guards::auth::AuthGuard;
use crate::db::connection::Pool;
use crate::auth_key::*;

use rocket_contrib::json::Json;
use diesel::prelude::*;
use rocket::State;

#[derive(Serialize)]
pub struct Status {
    status: String,
    key: AuthKey
}

impl Status {
    pub fn new_ok(status: &str, conn: &MysqlConnection) -> Self {
        Status {
            status: status.to_string(),
            key: generate_auth_key(conn)
        }
    }

    pub fn new_err(status: &str) -> Self {
        Status {
            status: status.to_string(),
            key: "".to_string()
        }
    }
}

fn save_value_to_db(db_key: &str, db_value: &str, conn: &MysqlConnection) -> Status {
    use crate::db::schema::nonrepeating::dsl::*;

    let res = diesel::update(nonrepeating.find(db_key))
        .set(
            value.eq(db_value)
        )
        .execute(conn);

    match res {
        Ok(_) => Status::new_ok("success", conn),
        Err(_) => Status::new_err("DB error")
    }
}

fn set_param(key: AuthKey, db_key: &str, db_value: &str, conn: State<Pool>) -> Json<Status> {
    let conn = conn.get().unwrap();

    Json(if !verify_auth_key(key, &conn) {
        Status::new_err("not logged in")
    } else {
        save_value_to_db(db_key, db_value, &conn)
    })
}

#[get("/setlogin")]
pub fn login(key: AuthGuard, value: ValueGuard, conn: State<Pool>)  -> Json<Status> {
    set_param(key.get(), "login", &value.get(), conn)
}

#[get("/setpass")]
pub fn password_hash(key: AuthGuard, value: ValueGuard, conn: State<Pool>) -> Json<Status> {
    set_param(key.get(), "pass_hash", &value.get(), conn)
}
