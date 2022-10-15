// Imports
use json::object;
use rocket::{http::Status, post, response::content::RawJson};

#[post("/login?<username>&<public_key>")]
pub fn login(username: &str, public_key: &str) -> RawJson<String> {
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

    // public_key is empty
    if public_key.is_empty() {
        return RawJson(
            object! {
                "code": bad.code,
                "message": "public_key is empty"
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
