use crate::connection::SafeConnection;
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
    use super::schema::nonrepeating::dsl::*;
    let db_key = "current_auth";

    let removal = diesel::delete(
            nonrepeating
                .filter(id.eq(db_key))
                .filter(title.eq(key))
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

#[get("/logout/<key>")]
pub fn logout(key: AuthKey, conn: State<SafeConnection>) -> Json<LogoutData> {
    let conn: &SafeConnection = &conn;
    let lock = (*conn).lock().unwrap();

    Json(remove_auth_key(key, &*lock))
}
