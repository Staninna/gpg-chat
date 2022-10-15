#[allow(dead_code)]
pub struct User {
    username: String,
    public_key: String,
}

#[allow(dead_code)]
impl User {
    pub fn new(username: String, public_key: String) -> User {
        User {
            username,
            public_key,
        }
    }
}
