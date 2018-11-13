use super::schema::posts;

#[derive(Queryable, Clone, Debug)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
