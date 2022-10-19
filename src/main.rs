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
    let appconfig = appconfig::appconfig();

    let connection = database::connect(&appconfig).await;

    let _rocket = rocket::build()
        .mount("/", FileServer::from(relative!("ui/")))
        .mount("/api/v1", routes![pong, register])
        .manage(appconfig)
        .manage(connection)
        .launch()
        .await?;

    Ok(())
}
