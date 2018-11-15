use crate::connection::SafeConnection;

use diesel::prelude::*;
use rocket::State;

fn remove_auth_key(conn: &MysqlConnection, key: &str) -> bool {
    use super::schema::nonrepeating::dsl::*;
    use diesel::dsl::exists;
    use diesel::dsl::select;

    let db_key = "current_auth";

    let should_delete = select(exists(
            nonrepeating
                .filter(id.eq(db_key))
                .filter(title.eq(key))
        ))
        .get_result(conn)
        .unwrap();

    if should_delete {
        diesel::delete(nonrepeating.filter(
                id.eq(db_key)
            ))
            .execute(conn)
            .unwrap();

        true
    } else {
        false
    }
}

#[post("/")]
pub fn logout(conn: State<SafeConnection>) -> String {
    let conn: &SafeConnection = &conn;
    let lock = (*conn).lock().unwrap();

    if remove_auth_key(&*lock, "") {
        "success".to_string()
    } else {
        "failed".to_string()
    }
}
