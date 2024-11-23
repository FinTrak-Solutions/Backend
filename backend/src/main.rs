#[macro_use] extern crate rocket;

mod routes;
mod models;
mod handlers;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Financial Tracker Backend!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![routes::auth::signup])
}
