use crate::dynamic_content;
use crate::static_content;

#[derive(Clone)]
pub struct Content {
    pub static_c: static_content::StaticContent,
    pub dynamic_c: dynamic_content::DynamicContent
}

impl Content {
    pub fn new(path: &str) -> Self {
        Content {
            static_c: static_content::StaticContent::new(path),
            dynamic_c: dynamic_content::DynamicContent::new()
        }
    }
}
