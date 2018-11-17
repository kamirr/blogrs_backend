pub use diesel::mysql::MysqlConnection;
pub use std::sync::{Arc, Mutex};
pub use diesel::prelude::*;
pub use diesel::r2d2;

use dotenv::dotenv;
use std::env;

pub type ConnectionManager = r2d2::ConnectionManager<MysqlConnection>;
pub type Pool = r2d2::Pool<ConnectionManager>;

pub fn make_pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .unwrap()
}
