use crate::connection::SafeConnection;
use crate::manage_posts::*;

use diesel::mysql::MysqlConnection;
use rocket_contrib::json::Json;
use rocket::State;

#[derive(Serialize, Deserialize)]
pub struct Meta {
    ids: Vec<u64>
}

impl Meta {
    fn from(conn: &MysqlConnection) -> Self {
        Meta {
            ids: all_ids(conn)
        }
    }
}

#[get("/meta")]
pub fn meta(conn: State<SafeConnection>) -> Json<Meta> {
    let lock = (*conn).lock().unwrap();
    Json(Meta::from(&*lock))
}
