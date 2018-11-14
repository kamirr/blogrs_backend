use crate::connection::SafeConnection;
use crate::models::Nonrepeating;
use crate::auth_key::*;

use rocket_contrib::json::Json;
use diesel::prelude::*;
use rocket::State;

#[derive(Serialize, Deserialize)]
pub struct LoginData {
    pub hash: String
}

fn test_hash(_l_hash: String, _p_hash: String, _data: LoginData) -> Option<AuthKey> {
    //TODO: Actually test hashes xd
    Some(generate_auth_key())
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

    drop(lock);

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
