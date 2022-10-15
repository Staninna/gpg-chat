// Imports
use super::user::User;

#[allow(dead_code)]
pub struct Client {
    user: User,
    client_id: String,
    connected: bool,
    ip: String,
}

#[allow(dead_code)]
impl Client {
    pub fn new(user: User, client_id: String, connected: bool, ip: String) -> Client {
        Client {
            user,
            client_id,
            connected,
            ip,
        }
    }

    pub fn get_user(&self) -> &User {
        &self.user
    }

    pub fn get_client_id(&self) -> &String {
        &self.client_id
    }

    pub fn get_ip(&self) -> &String {
        &self.ip
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}
