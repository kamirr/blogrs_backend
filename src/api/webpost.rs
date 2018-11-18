use crate::db::models::Post;

#[derive(Serialize, Deserialize)]
pub struct WebPost {
    pub title: String,
    pub body: String
}

impl WebPost {
    pub fn from_post(post: Post) -> Self {
        WebPost {
            title: post.title,
            body: post.body
        }
    }
}
