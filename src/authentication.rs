use sha2::{Sha512, Digest};
use std::path::Path;
use std::str;
use std::fs;

pub type AuthKey = String;

fn hash_text(text: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.input(text);
    hasher
        .result()
        .iter()
        .map(|n| format!("{:X?}", n))
        .collect::<Vec<String>>()
        .join("")
}

fn hash(login: &str, password: &str) -> String {
    let text = format!("login: {}; password: {}", login, password);
    hash_text(&text)
}

fn current_salt() -> String {
    match fs::read_to_string(Path::new("salt.txt")) {
        Ok(s) => s,
        Err(_) => "".to_string()
    }
}

fn current_pass() -> String {
    match fs::read_to_string(Path::new("pass.txt")) {
        Ok(s) => s,
        Err(_) => "".to_string()
    }
}

pub fn register(login: &str, password: &str) -> std::io::Result<()> {
    fs::write("pass.txt", hash(login, password))
}

pub fn random_hex(len: usize) -> String {
    let mut arr: Vec<u8> = vec![];
    for _ in 0 .. len {
        arr.push(rand::random::<u8>());
    }

    arr
        .iter()
        .map(|n| format!("{:X?}", n))
        .collect::<Vec<String>>()
        .join("")
}

pub fn get_salt() -> String {
    let res = random_hex(124);
    fs::write("salt.txt", &res).unwrap();

    //Helper; to be removed later on
    let salted = format!("{}{}", current_pass(), current_salt());
    println!("{}", hash_text(&salted));

    res
}

pub fn login(key: &str) -> Option<AuthKey> {
    let salted = format!("{}{}", current_pass(), current_salt());
    match key == hash_text(&salted) {
        true => Some(random_hex(124)),
        false => None
    }
}
