use crate::db::schema::nonrepeating;
use crate::db::schema::posts;

#[derive(Queryable, Clone, Debug)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String
}

#[derive(Insertable, Deserialize)]
#[table_name="posts"]
pub struct WebPost {
    pub title: String,
    pub body: String,
}

#[derive(Queryable, Insertable, Clone, Debug)]
#[table_name="nonrepeating"]
pub struct Nonrepeating {
    pub id: String,
    pub value: String
}
