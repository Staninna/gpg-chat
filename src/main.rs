// Rules
#![forbid(unsafe_code)]

// Modules
mod appconfig;
mod database;
mod v1;

// Imports
use crate::v1::routes::{auth::register, ping::pong};
use rocket::{
    fs::{relative, FileServer},
    routes,
};

// Main function
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Load appconfig.ini
    let appconfig = appconfig::appconfig();

    // Connect/set up database
    let conn = database::connect(&appconfig).await;
    database::setup(&conn).await;

    // Start rocket
    let _rocket = rocket::build()
        .mount("/", FileServer::from(relative!("ui/")))
        .mount("/api/v1", routes![pong, register])
        .manage(appconfig)
        .manage(conn)
        .launch()
        .await?;

    Ok(())
}
