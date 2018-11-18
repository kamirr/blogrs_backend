use crate::guards::login::LoginGuard;
use crate::db::models::Nonrepeating;
use crate::db::connection::Pool;
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
pub struct LoginStatus {
    pub status: String,
    pub key: AuthKey
}

impl LoginStatus {
    pub fn new_ok(status: &str, conn: &MysqlConnection) -> Self {
        LoginStatus {
            status: status.to_string(),
            key: generate_auth_key(conn)
        }
    }

    pub fn new_err(status: &str) -> Self {
        LoginStatus {
            status: status.to_string(),
            key: "".to_string()
        }
    }
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
    use crate::db::schema::nonrepeating::dsl::*;

    let res = nonrepeating
        .filter(id.eq(key))
        .load::<Nonrepeating>(conn);

    match res {
        Ok(v) => Ok(v),
        _ => Err(())
    }
}

fn test_login(hash: String, conn: &MysqlConnection) -> bool {
    let login_hash = match fetch_from_nonrepeating("login", conn) {
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

#[get("/login")]
pub fn login(lg: LoginGuard, conn: State<Pool>) -> Json<LoginStatus> {
    let conn = conn.get().unwrap();
    let hash = lg.hash;

    Json(if test_login(hash, &conn) {
        LoginStatus::new_ok("success", &conn)
    } else {
        LoginStatus::new_err("error")
    })
}