use crate::db::manage_posts::*;
use crate::db::connection::Pool;

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
pub fn meta(conn: State<Pool>) -> Json<Meta> {
    let conn = conn.get().unwrap();
    Json(Meta::from(&conn))
}
