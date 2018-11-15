use crate::connection::SafeConnection;
use crate::models::Nonrepeating;
use crate::auth_key::*;

use rocket_contrib::json::Json;
use sha2::{Sha224, Digest};
use diesel::prelude::*;
use rocket::State;

#[derive(Deserialize)]
pub struct LoginData {
    pub hash: String
}

#[derive(Serialize)]
pub struct ApiKey {
    pub key: AuthKey
}

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

fn fetch_from_nonrepeating(key: &str, conn: &MysqlConnection) -> Result<Vec<Nonrepeating>, ()> {
    use super::schema::nonrepeating::dsl::*;

    let res = nonrepeating
        .filter(id.eq(key))
        .load::<Nonrepeating>(conn);

    match res {
        Ok(v) => Ok(v),
        _ => Err(())
    }
}

fn test_login(hash: String, conn: &MysqlConnection) -> bool {
    let login_hash = match fetch_from_nonrepeating("login_hash", conn) {
        Ok(hash) => match hash.len() {
            1 => hash[0].value.clone(),
            _ => return true
        },
        Err(_) => return false
    };

    let pass_hash = match fetch_from_nonrepeating("pass_hash", conn) {
        Ok(hash) => match hash.len() {
            1 => hash[0].value.clone(),
            _ => return true
        },
        Err(_) => return false
    };

    test_hash(login_hash, pass_hash, hash)
}

#[get("/login/<hash>")]
pub fn login(hash: String, conn: State<SafeConnection>) -> Json<ApiKey> {
    let conn: &SafeConnection = &conn;
    let lock = (*conn).lock().unwrap();

    if test_login(hash, &*lock) {
        Json(ApiKey{
            key: generate_auth_key(&*lock)
        })
    } else {
        Json(ApiKey{
            key: "".into()
        })
    }
}
