use crate::db::DbPool;
use crate::models::account::Account;
use crate::models::category::Category;
use crate::models::transaction::{ClientTransaction, NewTransaction};
use crate::models::user::User;
use crate::schema::accounts::dsl::*;
use crate::schema::categories::dsl::*;
use crate::schema::transactions::dsl::*;
use crate::schema::users::dsl::{email as user_email, users}; // For users table
use chrono::prelude::*;
use diesel::prelude::*;
use rocket::http::Status;

pub async fn handle_add_transaction(
    new_trans: ClientTransaction,
    pool: DbPool,
) -> (Status, String) {
    // Step 1: Validate input
    if new_trans.email.is_empty() {
        return (Status::BadRequest, "Invalid input".to_string());
    }

    // Step 1.1: Check if the email exists in users table
    let user_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_check = new_trans.email.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            users
                .filter(user_email.eq(email_to_check))
                .first::<User>(&mut conn)
                .optional()
        }
    })
    .await;

    match user_exists {
        Ok(Ok(None)) => {
            // No user found for this email
            return (
                Status::BadRequest,
                "No user found for the provided email".to_string(),
            );
        }
        Ok(Ok(Some(_user))) => {
            // User found, proceed to category name existence check
        }
        Ok(Err(e)) => {
            eprintln!("Error checking user existence: {:?}", e);
            return (Status::InternalServerError, "Database error".to_string());
        }
        Err(e) => {
            eprintln!("Blocking task failed during user check: {:?}", e);
            return (
                Status::InternalServerError,
                "Internal server error".to_string(),
            );
        }
    }

    // Step 1.2: Check if the category_id exists
    let category_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let cat_to_check = new_trans.category_name.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            categories
                .filter(nickname.eq(cat_to_check))
                .first::<Category>(&mut conn)
                .optional()
        }
    })
    .await;

    let curr_cat_id = match category_exists {
        Ok(Ok(None)) => {
            // No user found for this email
            return (
                Status::BadRequest,
                "No category found for the provided email".to_string(),
            );
        }
        Ok(Ok(Some(match_category))) => {
            // User found, proceed to category name existence check
            match_category.category_id
        }
        Ok(Err(e)) => {
            eprintln!("Error checking category existence: {:?}", e);
            return (Status::InternalServerError, "Database error".to_string());
        }
        Err(e) => {
            eprintln!("Blocking task failed during user check: {:?}", e);
            return (
                Status::InternalServerError,
                "Internal server error".to_string(),
            );
        }
    };
    // Step 1.3: Check if account_id exists
    let account_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let acc_to_check = new_trans.account_name.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            accounts
                .filter(account_name.eq(acc_to_check))
                .first::<Account>(&mut conn)
                .optional()
        }
    })
    .await;

    let curr_acc_id = match account_exists {
        Ok(Ok(None)) => {
            return (
                Status::BadRequest,
                "No account found for the provided email".to_string(),
            );
        }
        Ok(Ok(Some(match_acc))) => {
            // User found, proceed to category name existence check
            match_acc.account_id
        }
        Ok(Err(e)) => {
            eprintln!("Error checking account existence: {:?}", e);
            return (Status::InternalServerError, "Database error".to_string());
        }
        Err(e) => {
            eprintln!("Blocking task failed during user check: {:?}", e);
            return (
                Status::InternalServerError,
                "Internal server error".to_string(),
            );
        }
    };

    // Step 2: construct new transaction to be added
    let db_new_trans = NewTransaction {
        email: new_trans.email.clone(),
        category_id: curr_cat_id.clone(),
        amount: new_trans.amount.clone(),
        notes: new_trans.notes.clone(),
        account_id: curr_acc_id.clone(),
        transaction_date: Utc::now().to_string(),
    };

    // Step 3: add new transaction to DB
    let result = tokio::task::spawn_blocking({
        let pool = pool.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            diesel::insert_into(transactions)
                .values(&db_new_trans) // Use one copy of the NewTransaction
                .execute(&mut conn)
        }
    })
    .await;

    match result {
        Ok(Ok(_)) => {
            // Successfully inserted the category
            let msg = "Successfully added transaction".to_string();
            (Status::Created, msg)
        }
        Ok(Err(e)) => {
            eprintln!("Database error during insertion: {:?}", e);
            (Status::InternalServerError, "Database error".to_string())
        }
        Err(e) => {
            eprintln!("Blocking task failed during insertion: {:?}", e);
            (
                Status::InternalServerError,
                "Internal server error".to_string(),
            )
        }
    }
}
