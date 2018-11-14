use crate::manage_posts::*;
use crate::connection::SafeConnection;
use diesel::mysql::MysqlConnection;
use rocket::State;

fn parse_entry_templ(entry: &String, id: u64, conn: &MysqlConnection) -> Option<String> {
    match fetch_post(conn, id) {
        Some(post) => {
            Some(entry
                .replace("{title}", &post.title)
                .replace("{body}", &post.body)
            )
        },
        _ => None
    }
}

#[get("/post/<id>")]
pub fn html_post(id: u64, conn: State<SafeConnection>) -> Option<String> {
    let entry = "frontend/templates/entry.html";
    match std::fs::read_to_string(entry) {
        Ok(text) => {
            let conn = &(*(*conn).lock().unwrap());
            parse_entry_templ(&text, id, conn)
        },
        _ => None
    }
}
