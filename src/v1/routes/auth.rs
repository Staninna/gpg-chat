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
    let new_user = User::new(username, public_key, password_hash);

    // Default status responses
    let intern_error = Status::InternalServerError;
    let bad = Status::BadRequest;
    let ok = Status::Ok;

    // Load variables from appconfig
    let username_regex = Regex::new(appconfig.get("username", "regex").unwrap().as_str()).unwrap();
    let username_regex_comment = appconfig.get("username", "comment").unwrap().to_string();
    let gpg_regex = Regex::new(appconfig.get("gpg", "regex").unwrap().as_str()).unwrap();
    let sha256_regex =
        Regex::new(appconfig.get("password", "sha256_regex").unwrap().as_str()).unwrap();

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
    if !username_regex.is_match(&new_user.username) {
        return RawJson(
            object! {
                "code": bad.code,
                "message": format!("Username doesn't match regex"),
                "comment": username_regex_comment,
                "regex": username_regex.as_str()
            }
            .dump(),
        );
    }

    // Username is already taken
    if usernames.contains(&new_user.username) {
        return RawJson(
            object! {
                "code": bad.code,
                "message": "Username is already taken"
            }
            .dump(),
        );
    }

    // Public key isn't valid
    if !gpg_regex.is_match(&new_user.public_key) {
        return RawJson(
            object! {
                "code": bad.code,
                "message": "Public key is not a valid GPG public key",
                "comment": "The public key must be in the format of a GPG public key block if you are sure it is, please open an issue on github and include the public key https://github.com/staninna/gpg-chat/issues/new ",
                "regex": gpg_regex.as_str(),
            }
            .dump(),
        );
    }

    // Password hash doesn't match regex
    if !sha256_regex.is_match(&new_user.password_hash) {
        return RawJson(
            object! {
                "code": bad.code,
                "message": "Password hash is not a valid SHA256 hash",
                "regex": sha256_regex.as_str()
            }
            .dump(),
        );
    }

    // Hash password with salt
    let password_salt = format!("{}{}", new_user.password_hash, salt);
    let raw_password_hash = Sha512::digest(password_salt.as_bytes());
    let password_hash = format!("{:x}", raw_password_hash);

    // Insert user into database
    let result = conn
        .call(move |conn| {
            conn.execute(
                "INSERT INTO users (username, public_key, password_hash) VALUES (?1, ?2, ?3)",
                params![new_user.username, new_user.public_key, password_hash],
            )
        })
        .await;

    // Respond with appropriate message
    match result {
        Ok(_) => {
            return RawJson(
                object! {
                    "code": ok.code,
                    "message": "User registered"
                }
                .dump(),
            );
        }
        Err(_) => {
            return RawJson(
                object! {
                    "code": intern_error.code,
                    "message": "Internal Server Error"
                }
                .dump(),
            );
        }
    }
}
