use crate::db::models::Nonrepeating;
use crate::db::connection::*;

use diesel::mysql::MysqlConnection;
use diesel::insert_into;
use diesel::prelude::*;
use rocket::State;
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
    use crate::db::schema::nonrepeating::dsl::*;
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

pub fn verify_auth_key(key: AuthKey, pool: &State<Pool>) -> bool {
    use crate::db::schema::nonrepeating::dsl::*;
    use diesel::dsl::exists;
    use diesel::dsl::select;

    let db_key = "current_auth";
    let conn = pool.get().unwrap();

    let ok = select(exists(
            nonrepeating
                .filter(id.eq(db_key))
                .filter(value.eq(key))
        ))
        .get_result(&conn)
        .unwrap();

    ok
}

pub fn generate_auth_key(pool: &State<Pool>) -> AuthKey {
    let res = random_hex(6);
    save_key_in_db(&res, &pool.get().unwrap());

    res
}
