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

    pub fn fetch_post(&mut self, id: &str) -> Option<String> {
        self.sc.fetch("frontent/templates/entry.html")
    }

    pub fn fetch(&mut self, file: &str) -> Option<String> {
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
