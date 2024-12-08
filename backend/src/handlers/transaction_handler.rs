use crate::db::DbPool;
use crate::models::account::Account;
use crate::models::category::Category;
use crate::models::transaction::{ClientTransaction, NewTransaction, Transaction};
use crate::models::user::User;
use crate::schema::accounts::dsl::*;
use crate::schema::categories::dsl::*;
use crate::schema::transactions::dsl::*;
use crate::schema::users::dsl::{email as user_email, users}; // For users table
use chrono::prelude::*;
use diesel::prelude::*;
use rocket::http::Status;

// POST add transaction
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
            let inserted_row: Result<Vec<i32>, diesel::result::Error> =
                diesel::insert_into(transactions)
                    .values(&db_new_trans) // Use one copy of the NewTransaction
                    .returning(trans_id)
                    .get_results(&mut conn);
            match inserted_row {
                Ok(row_ids) => return (Status::Created, row_ids[0].to_string()),
                Err(_) => return (Status::InternalServerError, "Database error".to_string()),
            }
        }
    })
    .await;

    match result {
        Err(_) => return (Status::InternalServerError, "Database error".to_string()),
        Ok(db_res) => return db_res,
    }
}

// DELETE delete transaction
pub async fn handle_delete_transaction(tx_id: String, pool: DbPool) -> (Status, &'static str) {
    // Check if email is empty or account_name is empty
    if tx_id.is_empty() {
        return (Status::BadRequest, "Invalid input");
    }

    let tx_id_int: i32 = tx_id.parse::<i32>().unwrap();
    // Check if transaction ID exists
    let tx_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            transactions
                .filter(trans_id.eq(tx_id_int))
                .first::<Transaction>(&mut conn)
                .optional()
        }
    })
    .await;

    match tx_exists {
        Ok(Ok(None)) => {
            // No transaction found for this ID
            return (
                Status::BadRequest,
                "No transaction found for the provided ID",
            );
        }
        Ok(Ok(Some(_))) => {
            // Transaction found, proceed with transaction deletion
            let deletion_result = tokio::task::spawn_blocking({
                let pool = pool.clone();
                move || {
                    let mut conn = pool.get().expect("Failed to get database connection");
                    diesel::delete(transactions.filter(trans_id.eq(tx_id_int))).execute(&mut conn)
                }
            })
            .await;

            match deletion_result {
                Ok(Ok(rows_deleted)) => {
                    if rows_deleted > 0 {
                        (Status::Ok, "Transaction successfully deleted")
                    } else {
                        (
                            Status::InternalServerError,
                            "Failed to delete the transaction",
                        )
                    }
                }
                Ok(Err(e)) => {
                    eprintln!("Error during deletion: {:?}", e);
                    (
                        Status::InternalServerError,
                        "Database error during deletion",
                    )
                }
                Err(e) => {
                    eprintln!("Blocking task failed during deletion: {:?}", e);
                    (Status::InternalServerError, "Internal server error")
                }
            }
        }
        Ok(Err(e)) => {
            eprintln!("Error checking user existence: {:?}", e);
            return (Status::InternalServerError, "Database error");
        }
        Err(e) => {
            eprintln!("Blocking task failed during user check: {:?}", e);
            return (Status::InternalServerError, "Internal server error");
        }
    }
}
