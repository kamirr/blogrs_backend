pub fn url(address: &str, port: u32, user: &str, password: &str, database: &str) -> String {
    format!("mysql://{}:{}@{}:{}/{}", user, password, address, port, database)
}
