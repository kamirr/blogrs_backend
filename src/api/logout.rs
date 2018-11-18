use crate::guards::auth::AuthGuard;
use crate::db::connection::Pool;
use crate::auth::*;

use rocket_contrib::json::JsonValue;
use diesel::prelude::*;
use rocket::State;

fn delete_auth_key_from_db(key: AuthKey, conn: &MysqlConnection) -> JsonValue {
    use crate::db::schema::nonrepeating::dsl::*;
    let db_key = "current_auth";

    let removal = diesel::delete(
            nonrepeating
                .filter(id.eq(db_key))
                .filter(value.eq(key))
        )
        .execute(conn);

    json!({"status": match removal {
        Ok(_) => "success",
        _ => "DB Error"
    }})
}

fn remove_auth_key(key: AuthKey, conn: &MysqlConnection) -> JsonValue {
    let ok = verify_auth_key(key.to_string(), conn);
    if ok {
        delete_auth_key_from_db(key, conn)
    } else {
        json!({"status": "not logged in"})
    }
}

#[get("/logout")]
pub fn logout(key: AuthGuard, conn: State<Pool>) -> JsonValue {
    let conn = conn.get().unwrap();
    remove_auth_key(key.get(), &conn)
}
