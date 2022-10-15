use rocket::{fs::FileServer, get, launch, routes, Build, Rocket};

#[get("/world")]
fn hello_world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", FileServer::from("ui/")) // <-- Hosts the frontend if I not gonna use any kind of frontend framework
        .mount("/hello", routes![hello_world])
}
