#[derive(Clone)]
pub struct DynamicContent { }

impl DynamicContent {
    pub fn new() -> Self {
        DynamicContent { }
    }

    pub fn fetch_post(&self, id: &String) -> Option<String> {
        Some(format!("post: {}", id))
    }

    pub fn fetch(self, file: String) -> Option<String> {
        let mut dirs: Vec<String> = file.split("/").map(|s| s.to_string()).collect();
        dirs.remove(0);

        println!("{:?}", dirs);

        if dirs.len() < 2 {
            return None;
        }

        match dirs[1].as_ref() {
            "post" => {
                if dirs.len() < 3 {
                    None
                } else {
                    self.fetch_post(&dirs[2])
                }
            },
            _ => None
        }
    }
}
