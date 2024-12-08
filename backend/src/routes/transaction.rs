use crate::db::DbPool;
use crate::handlers::transaction_handler;
use crate::models::transaction::ClientTransaction;
//use crate::models::transaction::NewTransaction;
//use crate::models::transaction::Transaction;
use rocket::http::Status;
use rocket::serde::json::Json;
#[allow(unused_imports)]
use rocket::serde::Serialize;
use rocket::State;

#[post("/add_trans", format = "json", data = "<new_trans>")]
pub async fn add_trans(
    new_trans: Json<ClientTransaction>,
    pool: &State<DbPool>,
) -> (Status, String) {
    transaction_handler::handle_add_transaction(new_trans.into_inner(), pool.inner().clone()).await
}
