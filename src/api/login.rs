use crate::guards::login::LoginGuard;
use crate::db::connection::Pool;
use crate::db::nonrepeating;
use crate::auth::*;

use rocket_contrib::json::{JsonValue};
use sha2::{Sha224, Digest};
use rocket::State;

fn test_hash(l_hash: String, p_hash: String, data: String) -> bool {
    let mut hasher = Sha224::new();
    hasher.input(format!("{}{}", l_hash, p_hash));

    let client_hash = data;
    let local_hash = hasher
        .result()
        .iter()
        .map(|n| {
            if *n < 16 {
                format!("0{:x?}", n)
            } else {
                format!("{:x?}", n)
            }
        })
        .collect::<String>();

    client_hash == local_hash[0..12]
}

fn test_login(hash: String, pool: &State<Pool>) -> bool {
    let nr = nonrepeating::Nonrepeating::new(&pool);

    let login_hash = match nr.get("login") {
        Some(hash) => hash,
        None => return false
    };

    let pass_hash = match nr.get("pass_hash") {
        Some(hash) => hash,
        None => return false
    };

    test_hash(login_hash, pass_hash, hash)
}

#[get("/login")]
pub fn login(lg: LoginGuard, pool: State<Pool>) -> JsonValue {
    if test_login(lg.hash, &pool) {
        json!({"status": "success", "key": generate_auth_key(&pool)})
    } else {
        json!({"status": "error"})
    }
}
