// Imports
use configparser::ini::Ini;
use json::object;
use regex::Regex;
use rocket::{http::Status, post, response::content::RawJson, State};
use tokio_rusqlite::Connection;

#[post("/register?<username>")]
pub async fn register(
    appconfig: &State<Ini>,
    conn: &State<Connection>,
    username: &str,
) -> RawJson<String> {
    // Default status responses
    let bad = Status::BadRequest;
    let ok = Status::Ok;

    // Load variables from appconfig
    let username_regex = Regex::new(appconfig.get("username", "regex").unwrap().as_str()).unwrap();
    let regex_comment = appconfig.get("username", "comment").unwrap().to_string();

    // Get all usernames from database
    let usernames = conn
        .call(|conn| {
            let mut stmt = conn.prepare("SELECT username FROM users").unwrap();
            stmt.query_map([], |row| row.get(0))
                .unwrap()
                .map(|r| r.unwrap())
                .collect::<Vec<String>>()
        })
        .await;

    // Username doesn't match regex
    if !username_regex.is_match(username) {
        return RawJson(
            object! {
                "code": bad.code,
                "message": format!("Username doesn't match regex"),
                "comment": regex_comment,
                "regex": username_regex.as_str()
            }
            .dump(),
        );
    }
    // Username is already taken
    else if usernames.contains(&username.to_string()) {
        return RawJson(
            object! {
                "code": bad.code,
                "message": "Username is already taken"
            }
            .dump(),
        );
    }
    // Username can not be Staninna because he is a bad person
    else if username.to_lowercase() == "staninna" {
        return RawJson(
            object! {
                "code": bad.code,
                "message": "Username can not be Staninna because he is a bad person"
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
