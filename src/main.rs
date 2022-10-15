use rocket::{
    fs::{relative, FileServer},
    get, launch, routes, Build, Rocket,
};

#[get("/ping")]
fn ping_pong() -> &'static str {
    "pong"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", FileServer::from(relative!("ui/"))) // <-- Hosts the frontend if I not gonna use any kind of frontend framework
        .mount("/", routes![ping_pong])
}
