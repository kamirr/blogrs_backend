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

pub fn fetch_post(conn: &MysqlConnection) -> Post {
    use super::schema::posts::dsl::*;

    let result = posts
        .limit(1)
        .load::<Post>(conn)
        .expect("Error loading posts");

    result[0].clone()
}
