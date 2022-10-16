#[rocket::get("/ping")]
pub fn pong() -> &'static str {
    "pong"
}
