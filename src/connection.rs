pub use diesel::mysql::MysqlConnection;
pub use std::sync::{Arc, Mutex};
pub use diesel::prelude::*;
pub use diesel::r2d2;

use dotenv::dotenv;
use std::env;

pub type ConnectionManager = r2d2::ConnectionManager<MysqlConnection>;
pub type Pool = r2d2::Pool<ConnectionManager>;

pub fn pool() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(database_url);
    let workers = num_cpus::get() as u32 * 2;

    println!("Establishing database connections:");
    let pool = r2d2::Pool::builder()
        .min_idle(Some(workers))
        .max_size(workers)
        .build(manager)
        .unwrap();

    println!("    => connections: {}", workers);

    pool
}
