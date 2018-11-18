pub use diesel::mysql::MysqlConnection;
pub use std::sync::{Arc, Mutex};
pub use diesel::prelude::*;
pub use diesel::r2d2;

use crate::db::url::url;

pub type ConnectionManager = r2d2::ConnectionManager<MysqlConnection>;
pub type Pool = r2d2::Pool<ConnectionManager>;

pub fn pool() -> Pool {
    let database_url = url(
        "mysql.agh.edu.pl", 3306,
        "koczurek", "cNuLjk0DMyydcnNW",
        "koczurek"
    );

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
