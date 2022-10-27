pub struct User {
    pub username: String,
    pub public_key: String,
    pub password_hash: String,
}

impl User {
    pub fn new(username: &str, public_key: &str, password_hash: &str) -> User {
        User {
            username: String::from(username),
            public_key: String::from(public_key),
            password_hash: String::from(password_hash),
        }
    }
}
