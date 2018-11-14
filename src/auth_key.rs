use crate::models::Nonrepeating;

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

pub fn generate_auth_key(conn: &MysqlConnection) -> AuthKey {
    use super::schema::nonrepeating::dsl::*;
    use diesel::dsl::exists;
    use diesel::dsl::select;

    let res = random_hex(64);
    let key = "current_auth";

    let to_insert = Nonrepeating {
        id: key.to_string(),
        title: res.clone()
    };

     let should_delete: bool = select(exists(
        nonrepeating
            .filter(id.eq(key))
        ))
        .get_result(conn)
        .unwrap();

    if should_delete {
        diesel::delete(nonrepeating.filter(
                id.eq(key)
            ))
            .execute(conn)
            .unwrap();
    }

    insert_into(nonrepeating)
        .values(&to_insert)
        .execute(conn)
        .unwrap();

    res
}
