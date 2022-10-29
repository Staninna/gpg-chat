// Modules
mod appconfig;
mod database;
mod v1;

// Imports
use rocket::routes;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    process::exit,
};
use v1::routes::{auth::register, ping::pong};

// Main function
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Load appconfig.ini
    let appconfig = appconfig::appconfig();

    // Check if salt is valid
    if appconfig.get("password", "salt").unwrap().to_string().len() < 10 {
        eprintln!("Please change the salt in appconfig.ini to a 10+ character long string");

        // Exit the program if salt is not stored in salt file
        if !Path::new(&appconfig.get("password", "salt_path").unwrap()).exists() {
            exit(1);
        }
    }
    // Write SALT as bytes to file
    if !Path::new(&appconfig.get("password", "salt_path").unwrap()).exists() {
        let mut salt_file = File::create(appconfig.get("password", "salt_path").unwrap()).unwrap();
        salt_file
            .write_all(
                appconfig
                    .get("password", "salt")
                    .unwrap()
                    .to_string()
                    .as_bytes(),
            )
            .unwrap();
    }

    // read salt to string and store it in var salt
    let mut salt_file = File::open(appconfig.get("password", "salt_path").unwrap()).unwrap();
    let mut salt = String::new();
    salt_file.read_to_string(&mut salt).unwrap();

    // Connect/set up database
    let conn = database::connect(&appconfig).await;
    database::setup(&conn).await;

    // Start rocket
    let _rocket = rocket::build()
        .mount("/api/v1", routes![pong, register])
        .manage(appconfig)
        .manage(salt)
        .manage(conn)
        .launch()
        .await?;

    Ok(())
}
