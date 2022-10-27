// Imports
use crate::v1::structures::user::User;
use configparser::ini::Ini;
use json::object;
use regex::Regex;
use rocket::{http::Status, post, response::content::RawJson, State};
use rusqlite::params;
use sha2::{Digest, Sha512};
use tokio_rusqlite::Connection;

// TODO; Change to use data instead of url params
// https://stackoverflow.com/questions/59964486/how-can-i-respond-to-a-post-request-containing-json-data-with-rocket
#[post("/register?<username>&<public_key>&<password_hash>")]
pub async fn register(
    appconfig: &State<Ini>,
    salt: &State<String>,
    conn: &State<Connection>,
    username: &str,
    public_key: &str,
    password_hash: &str,
) -> RawJson<String> {
    let user = User::new(username, public_key, password_hash);

    // Default status responses
    let intern_error = Status::InternalServerError;
    let bad = Status::BadRequest;
    let ok = Status::Ok;

    // Load variables from appconfig
    let username_regex = Regex::new(appconfig.get("username", "regex").unwrap().as_str()).unwrap();
    let regex_comment = appconfig.get("username", "comment").unwrap().to_string();
    let public_key_regex =
        Regex::new(appconfig.get("gpg", "public_regex").unwrap().as_str()).unwrap();

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
    if !username_regex.is_match(&user.username) {
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
    else if usernames.contains(&user.username) {
        return RawJson(
            object! {
                "code": bad.code,
                "message": "Username is already taken"
            }
            .dump(),
        );
    }

    // Public key doesn't match regex
    if !public_key_regex.is_match(&user.public_key) {
        return RawJson(
            object! {
                "code": bad.code,
                "message": "Public key is not a valid GPG public key",
                "regex": public_key_regex.as_str()
            }
            .dump(),
        );
    }

    // Hash password with salt
    let password_salt = format!("{}{}", user.password_hash, salt);
    let raw_password_hash = Sha512::digest(password_salt.as_bytes());
    let final_password_hash = format!("{:x}", raw_password_hash);

    // Insert user into database
    conn.call(move |conn| {
        match conn.execute(
            "INSERT INTO users (username, public_key, password_hash) VALUES (?1, ?2, ?3)",
            params![user.username, user.public_key, final_password_hash],
        ) {
            // User was created
            Ok(_) => {
                return RawJson(
                    object! {
                        "code": ok.code,
                        "message": "Account created"
                    }
                    .dump(),
                )
            }

            // Count not insert user into database
            Err(e) => {
                eprintln!("Couldn't create user, Error: {}", e);
                return RawJson(
                    object! {
                        "code": intern_error.code,
                        "message": "Couldn't create user"
                    }
                    .dump(),
                );
            }
        }
    })
    .await
}
