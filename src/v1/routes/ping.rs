use rocket::get;

#[get("/ping")]
pub fn pingpong() -> &'static str {
    "pong"
}
