// Imports
use crate::v1::structures::user::User;
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

    // Doesn't compile yet but work in progress
    // println!("Username: {}", username);
    // let _ = conn
    //     .call(|conn| {
    //         let mut stmt = conn.prepare("SELECT username FROM users").unwrap();
    //         let users = stmt
    //             .query_row([], |row| {
    //                 Ok(User {
    //                     id: row.get(0)?,
    //                     username: row.get(1)?,
    //                     public_key: row.get(2)?,
    //                     created_at: row.get(3)?
    //             })
    //             .unwrap();
    //         println!("{:?}", users);
    //     })
    //     .await;

    // Username is valid
    RawJson(
        object! {
            "code": ok.code,
            "message": "Username is valid"
        }
        .dump(),
    )
}
