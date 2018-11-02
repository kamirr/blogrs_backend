use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

#[derive(Clone)]
pub struct StaticContent {
    path: String,
    cache: Arc<Mutex<HashMap<String, String>>>
}

impl StaticContent {
    pub fn new(path: &str) -> Self {
        StaticContent {
            path: path.to_string(),
            cache: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn fetch(&mut self, mut file: String) -> Option<String> {
        if file == "/" {
            file = "/index.html".to_string();
        }

        let path = format!("{}{}", self.path, file);
        let mut res: Option<String> = None;

        'fetching: {
            // Disallow paths going backwards; return None
            if file.contains("..") {
                break 'fetching;
            }

            let lock = (*self.cache).lock().unwrap();
            let entry = (*lock).get(&path);

            // File is cached – return its contents
            if entry.is_some() {
                res = Some(entry.unwrap().clone());
                break 'fetching;
            }

            drop(entry);
            drop(lock);

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
            let mut lock = (*self.cache).lock().unwrap();
            (*lock).insert(path.clone(), contents.clone());
            drop(lock);
            res = Some(contents);
        }

        res
    }
}
