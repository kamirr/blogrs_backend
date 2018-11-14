use crate::connection::SafeConnection;
use crate::manage_posts::*;

use diesel::mysql::MysqlConnection;
use rocket_contrib::json::Json;
use rocket::State;

#[derive(Serialize, Deserialize)]
pub struct PostsInfo {
    ids: Vec<u64>
}

impl PostsInfo {
    fn from(conn: &MysqlConnection) -> Self {
        PostsInfo {
            ids: all_ids(conn)
        }
    }
}

#[get("/")]
pub fn posts_info(conn: State<SafeConnection>) -> Json<PostsInfo> {
    let lock = (*conn).lock().unwrap();
    Json(PostsInfo::from(&*lock))
}
