#[macro_use]
extern crate rocket;

mod db;
mod handlers;
mod models;
mod routes;
mod schema;

// ROUTES
use routes::account::{account_create, account_summary, delete_account};
use routes::auth::signup;
use routes::category::{category_create, category_summary, category_update, delete_category};

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
        .mount("/", routes![account_summary])
        .mount("/", routes![delete_account])
        .mount("/", routes![livereload_catcher])
        .mount("/", routes![category_create])
        .mount("/", routes![delete_category])
        .mount("/", routes![category_summary])
        .mount("/", routes![category_update])
}
