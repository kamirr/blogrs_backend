use crate::models::Nonrepeating;

use rocket::http::{Cookie, Cookies};
use diesel::mysql::MysqlConnection;
use diesel::insert_into;
use diesel::prelude::*;
use rand::random;

pub type AuthKey = String;

fn random_hex(count: u32) -> String {
    let mut res = AuthKey::new();
    for _ in 0 .. count {
        let n = random::<u8>();
        let s = if n < 16 {
            format!("0{:x?}", n)
        } else {
            format!("{:x?}", n)
        };

        res.push_str(&s);
    }

    res
}

fn save_key_in_db(key: &str, conn: &MysqlConnection) {
    use super::schema::nonrepeating::dsl::*;
    use diesel::dsl::exists;
    use diesel::dsl::select;

    let db_key = "current_auth";

    let to_insert = Nonrepeating {
        id: db_key.to_string(),
        title: key.into()
    };

    let should_delete = select(exists(
            nonrepeating.filter(id.eq(db_key))
        ))
        .get_result(conn)
        .unwrap();

    if should_delete {
        diesel::delete(nonrepeating.filter(
                id.eq(db_key)
            ))
            .execute(conn)
            .unwrap();
    }

    insert_into(nonrepeating)
        .values(&to_insert)
        .execute(conn)
        .unwrap();
}

fn save_key_in_cookie(key: &str, cookies: &mut Cookies) {
    let cookie = Cookie
        ::build("auth_key", key.to_owned())
        .secure(true)
        .finish();
    cookies.add_private(cookie);
}

pub fn generate_auth_key(conn: &MysqlConnection, cookies: &mut Cookies) {
    let res = random_hex(64);
    save_key_in_db(&res, conn);
    save_key_in_cookie(&res, cookies);
}
