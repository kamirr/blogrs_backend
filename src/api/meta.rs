use crate::db::connection::Pool;
use crate::db::manage_posts::*;

use rocket_contrib::json::JsonValue;
use rocket::State;

#[get("/meta")]
pub fn meta(pool: State<Pool>) -> JsonValue {
    json!({"ids": all_ids(&pool)})
}
