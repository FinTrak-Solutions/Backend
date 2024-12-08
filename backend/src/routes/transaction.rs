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

// For /delete_trans
#[derive(FromForm)]
pub struct DeleteTransQuery {
    pub trans_id: String,
}

// DELETE
#[delete("/delete_trans?<delete_query..>")]
pub async fn delete_trans(
    delete_query: DeleteTransQuery,
    pool: &State<DbPool>,
) -> (Status, &'static str) {
    transaction_handler::handle_delete_transaction(delete_query.trans_id, pool.inner().clone())
        .await
}
