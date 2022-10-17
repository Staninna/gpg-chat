// Rules
#![forbid(unsafe_code)]

// Modules
mod appconfig;
mod v1;

// Imports
use crate::v1::routes::{auth::login, ping::pong};
use appconfig::appconfig;
use rocket::{
    fs::{relative, FileServer},
    routes,
};

// Main function
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let appconfig = appconfig();

    let _rocket = rocket::build()
        .mount("/", FileServer::from(relative!("ui/")))
        .mount("/api/v1", routes![pong, login])
        .manage(appconfig)
        .launch()
        .await?;

    Ok(())
}
