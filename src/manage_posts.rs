pub use diesel::mysql::MysqlConnection;
use crate::models::{Post, NewPost};
pub use diesel::prelude::*;

pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str) {
    use super::schema::posts;

    let new_post = NewPost {
        title,
        body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Couldn't insert post!");
}

pub fn fetch_post(conn: &MysqlConnection, identifier: u64) -> Option<Post> {
    use super::schema::posts::dsl::*;

    let results = posts
        .filter(id.eq(identifier))
        .load::<Post>(conn)
        .expect("Error loading posts");

    match results.len() {
        1 => Some(results[0].clone()),
        _ => None
    }
}
