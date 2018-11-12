pub use diesel::mysql::MysqlConnection;
use crate::models::{NewPost};
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
