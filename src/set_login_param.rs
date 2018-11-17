use crate::connection::Pool;
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

fn save_value_to_db(login_hash: String, db_key: &str, conn: &MysqlConnection) -> Status {
    use crate::schema::nonrepeating::dsl::*;

    let res = diesel::update(nonrepeating.find(db_key))
        .set(
            value.eq(login_hash)
        )
        .execute(conn);

    match res {
        Ok(_) => Status::new_ok("ok", conn),
        Err(_) => Status::new_err("DB error")
    }
}

fn set_param(key: AuthKey, new_hash: String, db_key: &str, conn: State<Pool>) -> Json<Status> {
    let conn = conn.get().unwrap();

    Json(if !verify_auth_key(key, &conn) {
        Status::new_err("not logged in")
    } else {
        save_value_to_db(new_hash, db_key, &conn)
    })
}

#[post("/set/login/<key>", data = "<new_hash>")]
pub fn set_login(key: AuthKey, new_hash: String, conn: State<Pool>)  -> Json<Status> {
    set_param(key, new_hash, "login_hash", conn)
}

#[post("/set/password/<key>", data = "<new_hash>")]
pub fn set_password(key: AuthKey, new_hash: String, conn: State<Pool>) -> Json<Status> {
    set_param(key, new_hash, "pass_hash", conn)
}
