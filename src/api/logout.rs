use crate::api::auth_guard::AuthGuard;
use crate::connection::Pool;
use crate::auth_key::*;

use rocket_contrib::json::Json;
use diesel::prelude::*;
use rocket::State;

#[derive(Serialize)]
pub struct LogoutData {
    pub status: String,
    pub key: AuthKey
}

impl LogoutData {
    fn from_status(status: &str) -> Self {
        LogoutData {
            status: status.to_string(),
            key: "".to_string()
        }
    }
}

fn delete_auth_key_from_db(key: AuthKey, conn: &MysqlConnection) -> LogoutData {
    use crate::schema::nonrepeating::dsl::*;
    let db_key = "current_auth";

    let removal = diesel::delete(
            nonrepeating
                .filter(id.eq(db_key))
                .filter(value.eq(key))
        )
        .execute(conn);

    match removal {
        Ok(_) => LogoutData::from_status("success"),
        Err(_) => LogoutData::from_status("DB error")
    }
}

fn remove_auth_key(key: AuthKey, conn: &MysqlConnection) -> LogoutData {
    let ok = verify_auth_key(key.to_string(), conn);
    if ok {
        delete_auth_key_from_db(key, conn)
    } else {
        LogoutData::from_status("not logged in")
    }
}

#[get("/logout")]
pub fn logout(ag: AuthGuard, conn: State<Pool>) -> Json<LogoutData> {
    let conn = conn.get().unwrap();

    Json(remove_auth_key(ag.key, &conn))
}
