use crate::guards::auth::AuthGuard;
use crate::db::connection::Pool;
use crate::db::nonrepeating::*;
use crate::auth::*;

use rocket_contrib::json::JsonValue;
use rocket::State;

fn remove_auth_key(key: AuthKey, pool: &State<Pool>) -> JsonValue {
    let ok = verify_auth_key(key.to_string(), &pool);

    if ok {
        let nr = Nonrepeating::new(&pool);
        match nr.unset_pair("auth_key", &key) {
            Ok(_) => json!({"status": "success"}),
            Err(_) => json!({"status": "error"})
        }
    } else {
        json!({"status": "not logged in"})
    }
}

#[get("/logout")]
pub fn logout(key: AuthGuard, pool: State<Pool>) -> JsonValue {
    remove_auth_key(key.get(), &pool)
}
