// Rules
#![forbid(unsafe_code)]

// Imports
pub mod appconfig;
pub mod v1;
use rocket::{
    fs::{relative, FileServer},
    routes,
};
use v1::routes::{auth::login, ping::pong};

// Main function
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", FileServer::from(relative!("ui/")))
        .mount("/api/v1", routes![pong, login])
        .launch()
        .await?;

    Ok(())
}
