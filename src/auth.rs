use crate::db::connection::*;
use crate::db::special::*;

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

pub fn verify_auth_key(key: AuthKey, pool: &State<Pool>) -> bool {
    match Special::new(pool).get("current_auth") {
        Some(key_in_db) => key == key_in_db,
        None => false
    }
}

pub fn generate_auth_key(pool: &State<Pool>) -> AuthKey {
    let res = random_hex(6);
    Special::new(pool).set("current_auth", &res).unwrap();

    res
}
