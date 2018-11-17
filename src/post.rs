use crate::manage_posts_table::*;
use crate::connection::Pool;
use crate::webpost::WebPost;

use rocket_contrib::json::Json;
use rocket::State;

#[get("/post/<id>")]
pub fn post(id: u64, conn: State<Pool>) -> Option<Json<WebPost>> {
    let conn = conn.get().unwrap();

    fetch_post(&conn, id)
        .map(|p| Json(WebPost::from_post(p)))
}
