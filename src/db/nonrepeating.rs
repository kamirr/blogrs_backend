use crate::db::connection::Pool;
use crate::db::models;

use diesel::r2d2::{PooledConnection, ConnectionManager};
use diesel::prelude::*;

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
        use crate::db::schema::nonrepeating::dsl::*;

        let res = nonrepeating
            .filter(id.eq(key))
            .load::<models::Nonrepeating>(&self.conn);

        match res {
            Ok(v) => if v.len() == 1 {
                    Some(v[0].value.clone())
                } else {
                    None
                },
            _ => None
        }
    }

    pub fn set(&self, db_key: &str, db_value: &str) -> Result<(), ()> {
        use crate::db::schema::nonrepeating::dsl::*;

        let res = diesel::update(nonrepeating.find(db_key))
            .set(
                value.eq(db_value)
            )
            .execute(&self.conn);

        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }

    pub fn unset_pair(&self, db_key: &str, db_value: &str) -> Result<(), ()> {
        use crate::db::schema::nonrepeating::dsl::*;

        let removal = diesel::delete(
                nonrepeating
                    .filter(id.eq(db_key))
                    .filter(value.eq(db_value))
            )
            .execute(&self.conn);

        match removal {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }
}
