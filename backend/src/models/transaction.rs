use diesel::prelude::*;
use crate::schema::transactions;
use serde::{Deserialize, Serialize};

// Struct for querying users
// optional
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Transaction {
    pub trans_id: i32,
    pub email: String,
    pub category_id : i32,
    pub amount: f64,
    pub notes: Option<String>,
    pub account_id : i32,
}

// Struct for inserting new users
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub email: String,
    pub category_id : i32,
    pub amount: f64,
    pub notes: Option<String>,
    pub account_id : i32,
}
