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

fn save_key_in_db(key: &str, conn: &MysqlConnection) {
    use super::schema::nonrepeating::dsl::*;
    use diesel::dsl::exists;
    use diesel::dsl::select;

    let db_key = "current_auth";

    let to_insert = Nonrepeating {
        id: db_key.to_string(),
        value: key.into()
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

pub fn verify_auth_key(key: AuthKey, conn: &MysqlConnection) -> bool {
    use super::schema::nonrepeating::dsl::*;
    use diesel::dsl::exists;
    use diesel::dsl::select;

    let db_key = "current_auth";

    let ok = select(exists(
            nonrepeating
                .filter(id.eq(db_key))
                .filter(value.eq(key))
        ))
        .get_result(conn)
        .unwrap();

    ok
}

pub fn generate_auth_key(conn: &MysqlConnection) -> AuthKey {
    let res = random_hex(6);
    save_key_in_db(&res, conn);

    res
}
