pub struct User {
    pub username: String,
    pub public_key: String,
    pub password_hash: String,
}

impl User {
    pub fn new(username: &str, public_key: &str, password_hash: &str) -> User {
        User {
            username: username.to_string(),
            public_key: public_key.to_string(),
            password_hash: password_hash.to_string(),
        }
    }
}
