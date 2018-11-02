use std::collections::HashMap;
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

pub struct StaticContent {
    path: String,
    cache: HashMap<String, String>
}

impl StaticContent {
    pub fn new(path: &str) -> Self {
        StaticContent {
            path: path.to_string(),
            cache: HashMap::new()
        }
    }

    pub fn fetch(&mut self, file: String) -> Option<String> {
        let path = format!("{}{}", self.path, file);
        let mut res: Option<String> = None;

        'fetching: {
            // Disallow paths going backwards; return None
            if file.contains("..") {
                break 'fetching;
            }

            let entry = self.cache.get(&path);

            // File is cached – return its contents
            if entry.is_some() {
                res = Some(entry.unwrap().clone());
                break 'fetching;
            }

            // File doesn't exist – return None
            if !Path::new(&path).exists() {
                break 'fetching;
            }

            let file = File::open(path.clone());

            // Can't open file – return None
            if file.is_err() {
                break 'fetching;
            }

            let mut file = file.unwrap();
            let mut contents = String::new();

            // Can't read file – return None
            if file.read_to_string(&mut contents).is_err() {
                break 'fetching;
            }

            // Load file into cache and return its contents
            self.cache.insert(path.clone(), contents);
            res = Some(self.cache.get(&path).unwrap().clone())
        }

        res
    }
}
