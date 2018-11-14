pub use diesel::mysql::MysqlConnection;
pub use std::sync::{Arc, Mutex};
pub use diesel::prelude::*;

use dotenv::dotenv;
use std::env;

pub type SafeConnection = Arc<Mutex<MysqlConnection>>;

pub fn establish_sql_connection() -> SafeConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let conn = MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    Arc::new(Mutex::new(conn))
}
