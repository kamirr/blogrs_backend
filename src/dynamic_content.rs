use crate::static_content::StaticContent;

#[derive(Clone)]
pub struct DynamicContent {
    sc: StaticContent
}

impl DynamicContent {
    pub fn new(path: &str) -> Self {
        DynamicContent {
            sc: StaticContent::new(path)
        }
    }

    pub fn fetch_post_templ(&mut self, id: &str) -> Option<String> {
        match self.sc.fetch("templates/entry.html") {
            Some(text) => {
                let text = text.replace("{ID}", &format!("{}", id));
                Some(text)
            },
            None => None
        }
    }

    pub fn fetch(&mut self, file: &str) -> Option<String> {
        let mut dirs: Vec<String> = file.split("/").map(|s| s.to_string()).collect();
        dirs.remove(0);

        if dirs.len() < 2 {
            return None;
        }

        match dirs[1].as_ref() {
            "post_templ" => {
                if dirs.len() < 3 {
                    None
                } else {
                    self.fetch_post_templ(&dirs[2])
                }
            }
            _ => None
        }
    }
}
