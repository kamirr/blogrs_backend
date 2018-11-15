use crate::connection::SafeConnection;
use crate::manage_posts_table::*;
use crate::webpost::WebPost;
use crate::auth_key::*;

use diesel::mysql::MysqlConnection;
use rocket_contrib::json::Json;
use rocket::State;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
    pub key: AuthKey,
    pub id: u64
}

impl Status {
    pub fn new_ok(status: &str, id: u64, conn: &MysqlConnection) -> Self {
        Status {
            status: status.to_string(),
            id: id,
            key: generate_auth_key(conn)
        }
    }

    pub fn new_err(status: &str) -> Self {
        Status {
            status: status.to_string(),
            id: 0,
            key: "".to_string()
        }
    }
}

#[post("/new/<key>", data = "<post>", format = "json")]
pub fn new(key: AuthKey, post: Json<WebPost>, conn: State<SafeConnection>) -> Json<Status> {
    let conn: &SafeConnection = &conn;
    let lock = (*conn).lock().unwrap();

    if !verify_auth_key(key, &*lock) {
        return Json(Status::new_err("not logged in"))
    }

    let post = post.into_inner();

    Json(match create_post(&*lock, &post.title, &post.body) {
        Some(id) => Status::new_ok("success", id, &*lock),
        _ => Status::new_err("DB error")
    })
}

#[post("/edit/<key>/<id>", data = "<post>", format = "json")]
pub fn update(key: AuthKey, id: u64, post: Json<WebPost>, conn: State<SafeConnection>) -> Json<Status> {
    let conn: &SafeConnection = &conn;
    let lock = (*conn).lock().unwrap();

    if !verify_auth_key(key, &*lock) {
        return Json(Status::new_err("not logged in"))
    }

    let post = post.into_inner();

    Json(match update_post(&*lock, id, &post.title, &post.body) {
        Some(id) => Status::new_ok("success", id, &*lock),
        _ => Status::new_err("DB error")
    })
}
