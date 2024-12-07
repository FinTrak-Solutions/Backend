use crate::db::DbPool;
use crate::handlers::category_handler;
use crate::models::category::NewCategory;
use rocket::http::Status;
use rocket::serde::json::Json;
#[allow(unused_imports)]
use rocket::serde::Serialize;
use rocket::State;

#[post("/category_create", format = "json", data = "<new_cat>")]
pub async fn category_create(new_cat: Json<NewCategory>, pool: &State<DbPool>) -> (Status, String) {
    category_handler::handle_category_create(new_cat.into_inner(), pool.inner().clone()).await
}
