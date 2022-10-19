// Rules
#![forbid(unsafe_code)]

// Modules
mod appconfig;
mod v1;

// Imports
use crate::v1::routes::{auth::register, ping::pong};
use appconfig::appconfig;
use rocket::{
    fs::{relative, FileServer},
    routes,
};
use std::process::exit;
use tokio_rusqlite::Connection;

// Main function
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let appconfig = appconfig();

    // Connect to database file or
    let conn = match appconfig.getbool("database", "file") {
        Ok(Some(true)) => Connection::open(appconfig.get("database", "path").unwrap())
            .await
            .unwrap(),
        Ok(Some(false)) => Connection::open_in_memory().await.unwrap(),
        _ => {
            eprintln!("Couldn't read database.file from appconfig.ini");
            exit(1);
        }
    };

    let _rocket = rocket::build()
        .mount("/", FileServer::from(relative!("ui/")))
        .mount("/api/v1", routes![pong, register])
        .manage(appconfig)
        .manage(conn)
        .launch()
        .await?;

    Ok(())
}
