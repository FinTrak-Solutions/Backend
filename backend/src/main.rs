#[macro_use] extern crate rocket;

mod routes;
mod models;
mod handlers;
mod db;
mod schema;

// ROUTES
use routes::auth::signup;


#[get("/")]
fn index() -> &'static str {
    "Welcome to Financial Tracker Backend!"
}

#[launch]
fn rocket() -> _ {
    let pool = db::establish_connection();

    rocket::build()
        .manage(pool)
        .mount("/", routes![index])
        .mount("/", routes![signup])
}
