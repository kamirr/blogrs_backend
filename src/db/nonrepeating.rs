use crate::db::connection::Pool;
use crate::db::models;

use diesel::r2d2::{PooledConnection, ConnectionManager};
use diesel::prelude::*;

fn fetch_from_nonrepeating(key: &str, conn: &MysqlConnection) -> Result<Vec<models::Nonrepeating>, ()> {
    use crate::db::schema::nonrepeating::dsl::*;

    let res = nonrepeating
        .filter(id.eq(key))
        .load::<models::Nonrepeating>(conn);

    match res {
        Ok(v) => Ok(v),
        _ => Err(())
    }
}

fn save_value_to_db(db_key: &str, db_value: &str, conn: &MysqlConnection) -> Result<(), ()> {
    use crate::db::schema::nonrepeating::dsl::*;

    let res = diesel::update(nonrepeating.find(db_key))
        .set(
            value.eq(db_value)
        )
        .execute(conn);

    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(())
    }
}

pub struct Nonrepeating {
    conn: PooledConnection<ConnectionManager<MysqlConnection>>
}

impl Nonrepeating {
    pub fn new(pool: &Pool) -> Self {
        Nonrepeating {
            conn: pool.get().unwrap()
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match fetch_from_nonrepeating(key, &self.conn) {
            Ok(hash) => match hash.len() {
                1 => Some(hash[0].value.clone()),
                _ => None
            },
            Err(_) => None
        }
    }

    pub fn set(&self, key: &str, value: &str) -> Result<(), ()> {
        save_value_to_db(key, value, &self.conn)
    }
}
