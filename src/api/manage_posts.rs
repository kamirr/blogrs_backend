use crate::guards::auth::AuthGuard;
use crate::db::connection::Pool;
use crate::db::manage_posts::*;
use crate::db::models::WebPost;
use crate::auth::*;

use rocket_contrib::json::{Json, JsonValue};
use diesel::mysql::MysqlConnection;
use rocket::State;

fn status_ok(status: &str, id: u64, conn: &MysqlConnection) -> JsonValue {
    json!({
        "status": status.to_string(),
        "post_id": id.to_string(),
        "key": generate_auth_key(conn)
    })
}

fn status_err(status: &str) -> JsonValue {
    json!({
        "status": status.to_string()
    })
}

#[post("/new", data = "<post>", format = "json")]
pub fn new(key: AuthGuard, post: Json<WebPost>, conn: State<Pool>) -> JsonValue {
    let conn = conn.get().unwrap();

    if !verify_auth_key(key.get(), &conn) {
        return status_err("not logged in")
    }

    let post = post.into_inner();

    match create_post(&conn, &post.title, &post.body) {
        Some(id) => status_ok("success", id, &conn),
        _ => status_err("DB error")
    }
}

#[post("/edit/<id>", data = "<post>", format = "json")]
pub fn update(key: AuthGuard, id: u64, post: Json<WebPost>, conn: State<Pool>) -> JsonValue {
    let conn = conn.get().unwrap();

    if !verify_auth_key(key.get(), &conn) {
        return status_err("not logged in")
    }

    let post = post.into_inner();

    match update_post(&conn, id, &post.title, &post.body) {
        Some(id) => status_ok("success", id, &conn),
        _ => status_err("DB error")
    }
}

#[get("/delete/<id>")]
pub fn delete(key: AuthGuard, id: u64, conn: State<Pool>) -> JsonValue {
    let conn = conn.get().unwrap();

    if !verify_auth_key(key.get(), &conn) {
        return status_err("not logged in");
    }

    match delete_post(&conn, id) {
        Ok(_) => status_ok("success", 0, &conn),
        Err(_) => status_err("DB error")
    }
}

#[get("/post/<id>")]
pub fn get(id: u64, conn: State<Pool>) -> Option<JsonValue> {
    let conn = conn.get().unwrap();

    fetch_post(&conn, id)
        .map(|p| json!({"title": p.title, "body": p.body}))
}
