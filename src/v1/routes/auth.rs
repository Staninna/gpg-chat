// Imports
use configparser::ini::Ini;
use json::object;
use regex::Regex;
use rocket::{http::Status, post, response::content::RawJson, State};

#[post("/login?<username>")]
pub fn login(appconfig: &State<Ini>, username: &str) -> RawJson<String> {
    // Default status responses
    let bad = Status::BadRequest;
    let ok = Status::Ok;

    // Load variables from appconfig
    let min_username_length: usize =
        appconfig.getint("username", "min_length").unwrap().unwrap() as usize;
    let max_username_length: usize =
        appconfig.getint("username", "max_length").unwrap().unwrap() as usize;
    let username_regex = Regex::new(appconfig.get("username", "regex").unwrap().as_str()).unwrap();

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
    else if username.len() <= min_username_length {
        return RawJson(
            object! {
                "code": ok.code,
                "message": "Username is too short"
            }
            .dump(),
        );
    }
    // username is too long
    else if username.len() >= max_username_length {
        return RawJson(
            object! {
                "code": ok.code,
                "message": "Username is too long"
            }
            .dump(),
        );
    }
    // Username doesn't match regex
    else if !username_regex.is_match(username) {
        return RawJson(
            object! {
                "code": bad.code,
                "message": format!("Username doesn't match regex"),
                "regex": username_regex.as_str()
            }
            .dump(),
        );
    }

    // Username is valid
    RawJson(
        object! {
            "code": ok.code,
            "message": "Username is valid"
        }
        .dump(),
    )
}
