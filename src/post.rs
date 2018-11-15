use crate::connection::SafeConnection;
use crate::webpost::WebPost;
use crate::manage_posts::*;

use rocket_contrib::json::Json;
use rocket::State;

#[get("/post/<id>")]
pub fn post(id: u64, conn: State<SafeConnection>) -> Option<Json<WebPost>> {
    let conn: &SafeConnection = &conn;
    let lock = (*conn).lock().unwrap();

    fetch_post(&*lock, id)
        .map(|p| Json(WebPost::from_post(p)))
}
