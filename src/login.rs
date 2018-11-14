use crate::connection::SafeConnection;
use crate::models::Nonrepeating;
use crate::auth_key::*;

use rocket_contrib::json::Json;
use sha2::{Sha224, Digest};
use diesel::prelude::*;
use rocket::State;

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub hash: String
}

fn test_hash(l_hash: String, p_hash: String, data: LoginData) -> Option<AuthKey> {
    let mut hasher = Sha224::new();
    hasher.input(format!("{}{}", l_hash, p_hash));

    let client_hash = data.hash;
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

    match client_hash == local_hash {
        true => Some(generate_auth_key()),
        false => None
    }
}

#[post("/", data = "<data>", format = "json")]
pub fn login(data: Json<LoginData>, conn: State<SafeConnection>) -> Option<AuthKey> {
    use super::schema::nonrepeating::dsl::*;

    let lock = (*conn).lock().unwrap();

    let login_hash = nonrepeating
        .filter(id.eq("login_hash"))
        .load::<Nonrepeating>(&*lock);

    let pass_hash = nonrepeating
        .filter(id.eq("pass_hash"))
        .load::<Nonrepeating>(&*lock);

    let login_hash = match login_hash {
        Ok(hash) => match hash.len() {
            1 => hash[0].title.clone(),
            _ => return Some(generate_auth_key())
        },
        Err(_) => return None
    };

    let pass_hash = match pass_hash {
        Ok(hash) => match hash.len() {
            1 => hash[0].title.clone(),
            _ => return Some(generate_auth_key())
        },
        Err(_) => return None
    };

    test_hash(login_hash, pass_hash, data.into_inner())
}
