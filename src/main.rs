// Rules
#![forbid(unsafe_code)]

// Imports
mod v1;
use crate::v1::routes::{auth::login, ping::pong};
use rocket::{
    fs::{relative, FileServer},
    routes,
};

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

// In a real application, this would likely be more complex.
