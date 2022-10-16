// Imports
use crate::appconfig::{MAX_LEN_USERNAME, MIN_LEN_USERNAME};
use json::object;
use rocket::{http::Status, post, response::content::RawJson};

#[post("/login?<username>")]
pub fn login(username: &str) -> RawJson<String> {
    let bad = Status::BadRequest;
    let ok = Status::Ok;
    let min_len_username = MIN_LEN_USERNAME - 1;

    match username.len() {
        // username is empty
        0 => RawJson(
            object! {
                "code": bad.code,
                "message": "Username can not be empty"
            }
            .dump(),
        ),
        // username is too short
        0..=min_len_username => RawJson(
            object! {
                "code": ok.code,
                "message": format!("Username of {} is too short", username.len())
            }
            .dump(),
        ),
        // username is too long
        MAX_LEN_USERNAME.. => RawJson(
            object! {
                "code": ok.code,
                "message": format!("Username of {} is too long", username.len())
            }
            .dump(),
        ),
        // username is valid
        _ => RawJson(
            object! {
                "code": ok.code,
                "message": "Successfully logged in with username"
            }
            .dump(),
        ),
    }
}
