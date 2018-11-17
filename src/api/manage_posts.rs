use crate::api::auth_guard::AuthGuard;
use crate::manage_posts_table::*;
use crate::connection::Pool;
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

#[post("/new", data = "<post>", format = "json")]
pub fn new(ag: AuthGuard, post: Json<WebPost>, conn: State<Pool>) -> Json<Status> {
    let conn = conn.get().unwrap();

    if !verify_auth_key(ag.key, &conn) {
        return Json(Status::new_err("not logged in"))
    }

    let post = post.into_inner();

    Json(match create_post(&conn, &post.title, &post.body) {
        Some(id) => Status::new_ok("success", id, &conn),
        _ => Status::new_err("DB error")
    })
}

#[post("/edit/<id>", data = "<post>", format = "json")]
pub fn update(ag: AuthGuard, id: u64, post: Json<WebPost>, conn: State<Pool>) -> Json<Status> {
    let conn = conn.get().unwrap();

    if !verify_auth_key(ag.key, &conn) {
        return Json(Status::new_err("not logged in"))
    }

    let post = post.into_inner();

    Json(match update_post(&conn, id, &post.title, &post.body) {
        Some(id) => Status::new_ok("success", id, &conn),
        _ => Status::new_err("DB error")
    })
}

#[get("/delete/<id>")]
pub fn delete(ag: AuthGuard, id: u64, conn: State<Pool>) -> Json<Status> {
    let conn = conn.get().unwrap();

    if !verify_auth_key(ag.key, &conn) {
        return Json(Status::new_err("not logged in"))
    }

    Json(match delete_post(&conn, id) {
        Ok(_) => Status::new_ok("success", 0, &conn),
        Err(_) => Status::new_err("DB error")
    })
}

#[get("/post/<id>")]
pub fn get(id: u64, conn: State<Pool>) -> Option<Json<WebPost>> {
    let conn = conn.get().unwrap();

    fetch_post(&conn, id)
        .map(|p| Json(WebPost::from_post(p)))
}
