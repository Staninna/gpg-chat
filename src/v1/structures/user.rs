#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub public_key: String, // TODO: Change to public key type
    pub created_at: String, // TODO: Change to DateTime type
}

#[allow(dead_code)]
impl User {
    pub fn new(id: i32, username: &str, public_key: &str, created_at: &str) -> User {
        User {
            id,
            username: String::from(username),
            public_key: String::from(public_key),
            created_at: String::from(created_at),
        }
    }
}
