// Imports
use json::object;
use rocket::{http::Status, post, response::content::RawJson};

#[post("/login?<username>")]
pub fn login(username: &str) -> RawJson<String> {
    let bad = Status::BadRequest;
    let ok = Status::Ok;

    // username is empty
    if username.is_empty() {
        return RawJson(
            object! {
                "code": bad.code,
                "message": "Username is empty"
            }
            .dump(),
        );
    }
    // username is to short
    else if username.len() < 5 {
        return RawJson(
            object! {
                "code": bad.code,
                "message": "Username is to short"
            }
            .dump(),
        );
    }
    // username is to long
    else if username.len() > 20 {
        return RawJson(
            object! {
                "code": bad.code,
                "message": "Username is too long"
            }
            .dump(),
        );
    }

    RawJson(
        object! {
            "code": ok.code,
            "message": "Successfully logged in"
        }
        .dump(),
    )
}
