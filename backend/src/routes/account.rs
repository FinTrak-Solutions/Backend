use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use crate::models::account::NewAccount;
use crate::db::DbPool;
use crate::handlers::account_handler;

#[post("/account_create", format = "json", data = "<new_acc>")]
pub async fn account_create(new_acc: Json<NewAccount>, pool: &State<DbPool>) -> (Status, String) {
    account_handler::handle_account_create(new_acc.into_inner(), pool.inner().clone()).await
}
