use crate::connection::SafeConnection;
use crate::manage_posts::*;
use crate::models::Post;

use rocket_contrib::json::Json;
use rocket::State;

#[get("/post/<id>")]
pub fn post(id: u64, conn: State<SafeConnection>) -> Json<Post> {
    let conn: &SafeConnection = &conn;
    let lock = (*conn).lock().unwrap();

    match fetch_post(&*lock, id) {
        Some(post) => Json(post),
        _ => Json(Post{
            id: 0,
            title: "".to_string(),
            body: "".to_string()
        })
    }
}
