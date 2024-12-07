#[macro_use] extern crate rocket;

mod routes;
mod models;
mod handlers;
mod db;
mod schema;

use routes::auth::signup;
use routes::account::account_create;

#[get("/livereload/<_..>")]
fn livereload_catcher() -> &'static str {
    "LiveReload route placeholder"
}

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
        .mount("/", routes![account_create])
        .mount("/", routes![livereload_catcher])
}
