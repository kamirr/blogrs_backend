use rand::random;

pub type AuthKey = String;

fn random_hex(count: u32) -> String {
    let mut res = AuthKey::new();
    for _ in 0 .. count {
        let n = random::<u8>();
        let s = if n < 16 {
            format!("0{:X?}", n)
        } else {
            format!("{:X?}", n)
        };

        res.push_str(&s);
    }

    res
}

pub fn generate_auth_key() -> AuthKey {
    random_hex(64)
}
