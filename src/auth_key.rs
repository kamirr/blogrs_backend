use rand::random;

pub type AuthKey = String;

fn random_hex(count: u32) -> String {
    let mut res = AuthKey::new();
    for _ in 0 .. count {
        res.push_str(&format!("{:X?}", random::<u8>()));
    }

    res
}

pub fn generate_auth_key() -> AuthKey {
    random_hex(64)
}
