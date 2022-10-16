// Rules
#![forbid(unsafe_code)]

// Imports
use rocket::{
    fs::{relative, FileServer},
    routes,
};

pub mod v1 {
    // Import structures
    pub mod structures {
        pub mod client;
        pub mod user;
    }
    // Import routes
    pub mod routes {
        pub mod auth;
        pub mod ping;
    }
}

// Main function
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", FileServer::from(relative!("ui/"))) // <-- Hosts the frontend if I not gonna use any kind of frontend framework
        .mount("/api/v1", routes![v1::routes::ping::pingpong]) // <-- Test route
        .mount("/api/v1", routes![v1::routes::auth::login]) // <-- Login route
        .launch()
        .await?;

    Ok(())
}
