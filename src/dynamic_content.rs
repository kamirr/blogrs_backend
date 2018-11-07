#[derive(Clone)]
pub struct DynamicContent {
    path: String
}

impl DynamicContent {
    pub fn new(path: &str) -> Self {
        DynamicContent {
            path: path.to_string()
        }
    }

    pub fn fetch_post(&self, id: &String) -> Option<String> {
        Some("text".to_string())
    }

    pub fn fetch(self, mut file: String) -> Option<String> {
        let dirs: Vec<String> = file.split("/").map(|s| s.to_string()).collect();

        if dirs.len() < 2 {
            return None;
        }

        match dirs[1].as_ref() {
            "post" => {
                if dirs.len() < 3 {
                    None
                } else {
                    self.fetch_post(&dirs[3])
                }
            },
            _ => None
        }
    }
}
