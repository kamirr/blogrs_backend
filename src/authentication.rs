use sha2::{Sha512, Digest};
use std::str;
use std::fs;

fn hash(login: &str, password: &str) -> String {
    let text = format!("login: {}; password: {}", login, password);

    let mut hasher = Sha512::new();
    hasher.input(text);
    hasher
        .result()
        .iter()
        .map(|n| format!("{:X?}", n))
        .collect::<Vec<String>>()
        .join("")
}

pub fn register(login: &str, password: &str) -> std::io::Result<()> {
    fs::write("pass.txt", hash(login, password))
}

pub fn get_salt() -> String {
    let mut arr: Vec<u8> = vec![];
    for _ in 0 .. 124 {
        arr.push(rand::random::<u8>());
    }

    let res = arr
        .iter()
        .map(|n| format!("{:X?}", n))
        .collect::<Vec<String>>()
        .join("");

    fs::write("salt.txt", &res).unwrap();
    res
}
