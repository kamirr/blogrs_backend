use crate::db::connection::Pool;
use crate::db::manage_posts::*;

use rocket_contrib::json::JsonValue;
use rocket::State;

#[get("/meta")]
pub fn meta(conn: State<Pool>) -> JsonValue {
    let conn = conn.get().unwrap();
    json!({"ids": all_ids(&conn)})
}
