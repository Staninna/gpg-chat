// Imports
use json::object;
use rocket::{http::Status, post, response::content::RawJson};

#[post("/login?<username>")]
pub fn login(username: &str) -> RawJson<String> {
    let bad = Status::BadRequest;
    let ok = Status::Ok;
    let min_username = 5;
    let max_username = 15;

    // username is empty
    if username.is_empty() {
        return RawJson(
            object! {
                "code": bad.code,
                "message": "Username can not be empty"
            }
            .dump(),
        );
    }
    // username is too short
    else if username.len() <= min_username {
        return RawJson(
            object! {
                "code": ok.code,
                "message": format!("Username of {} is too short", username.len())
            }
            .dump(),
        );
    }
    // username is too long
    else if username.len() >= max_username {
        return RawJson(
            object! {
                "code": ok.code,
                "message": format!("Username of {} is too long", username.len())
            }
            .dump(),
        );
    }

    // username is valid
    RawJson(
        object! {
            "code": ok.code,
            "message": "Successfully logged in with username"
        }
        .dump(),
    )
}
