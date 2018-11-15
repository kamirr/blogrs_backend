use crate::models::{Post, NewPost};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

#[allow(dead_code)]
pub fn create_post(conn: &MysqlConnection, title: &str, body: &str) -> Option<u64> {
    use super::schema::posts::dsl::posts;

    let new_post = NewPost {
        title,
        body,
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(conn);

    Some(0) // placeholder
}

#[allow(dead_code)]
pub fn update_post(conn: &MysqlConnection, identifier: u64, new_title: &str, new_body: &str) -> QueryResult<usize> {
    use crate::schema::posts::dsl::*;

    diesel::update(posts.find(identifier))
        .set((
            body.eq(new_body),
            title.eq(new_title)
        ))
        .execute(conn)
}

#[allow(dead_code)]
pub fn delete_post(conn: &MysqlConnection, identifier: u64) -> QueryResult<usize> {
    use crate::schema::posts::dsl::*;

    diesel::delete(
            posts.filter(
                id.eq(identifier)
            )
        )
        .execute(conn)
}

pub fn fetch_post(conn: &MysqlConnection, identifier: u64) -> Option<Post> {
    use super::schema::posts::dsl::*;

    let results = posts
        .filter(id.eq(identifier))
        .load::<Post>(conn);

    match results {
        Ok(res) =>
            match res.len() {
                1 => Some(res[0].clone()),
                _ => None
            },
        Err(_) => None
    }
}

pub fn all_ids(conn: &MysqlConnection) -> Vec<u64> {
    use super::schema::posts::dsl::*;

    let res = posts
        .select(id)
        .load::<u64>(conn);

    match res {
        Ok(res) => res,
        Err(_) => vec![]
    }
}
