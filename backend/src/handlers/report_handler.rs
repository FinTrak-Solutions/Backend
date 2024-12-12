use crate::db::DbPool;
use crate::models::user::User;
use crate::schema::transactions::dsl::*;
use crate::schema::users::dsl::{email as user_email, users}; // For users table
use diesel::dsl::sum;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

// Some helper functions
pub async fn check_email_valid(email_str: String, pool: DbPool) -> (Status, String) {
    let user_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_check = email_str.clone();
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
        Ok(Ok(Some(user))) => {
            // User found, proceed to category name existence check
            return (Status::Ok, user.email);
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
}

// GET /report_overview?email=<>
pub async fn handle_report_overview(
    email_str: String,
    pool: DbPool,
) -> (Status, Json<Vec<String>>) {
    // Step 1: validate email
    let (email_status, _user_email) = check_email_valid(email_str.clone(), pool.clone()).await;

    if email_status != Status::Ok {
        return (Status::BadRequest, Json(vec![]));
    }

    // Declare summary vector
    let mut summary: Vec<String> = vec!["Category Summary:".to_string()];

    // Step 2: get summary of category
    let category_result = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_search = email_str.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            // order matters: https://stackoverflow.com/questions/72670161/how-do-you-use-rust-diesel-to-do-a-group-by-query
            // nullable types: https://docs.diesel.rs/1.4.x/diesel/sql_types/struct.Nullable.html
            transactions
                .filter(crate::schema::transactions::dsl::email.eq(email_to_search))
                .group_by((crate::schema::transactions::dsl::category_id,))
                .select((
                    crate::schema::transactions::dsl::category_id,
                    sum(crate::schema::transactions::dsl::amount),
                ))
                .load::<(i32, Option<f64>)>(&mut conn)
        }
    })
    .await;

    match category_result {
        Ok(Ok(trans_list)) => {
            // Successfully retrieved accounts
            for &(cat_id, cat_sum) in trans_list.iter() {
                if let Some(valid_sum) = cat_sum {
                    let cat_line = format!("{}: {}", cat_id, valid_sum);
                    summary.push(cat_line);
                }
            }
        }
        Ok(Err(e)) => {
            eprintln!("Database error during category summary retrieval: {:?}", e);
            return (Status::InternalServerError, Json(vec![]));
        }
        Err(e) => {
            eprintln!(
                "Blocking task failed during category summary retrieval: {:?}",
                e
            );
            return (Status::InternalServerError, Json(vec![]));
        }
    }

    summary.push("Account Summary:".to_string());

    // Add account summary section
    let account_result = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_search = email_str.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            // order matters: https://stackoverflow.com/questions/72670161/how-do-you-use-rust-diesel-to-do-a-group-by-query
            // nullable types: https://docs.diesel.rs/1.4.x/diesel/sql_types/struct.Nullable.html
            transactions
                .filter(crate::schema::transactions::dsl::email.eq(email_to_search))
                .group_by((crate::schema::transactions::dsl::account_id,))
                .select((
                    crate::schema::transactions::dsl::account_id,
                    sum(crate::schema::transactions::dsl::amount),
                ))
                .load::<(i32, Option<f64>)>(&mut conn)
        }
    })
    .await;

    match account_result {
        Ok(Ok(trans_list)) => {
            // Successfully retrieved accounts
            for &(acc_id, acc_sum) in trans_list.iter() {
                if let Some(valid_sum) = acc_sum {
                    let acc_line = format!("{}: {}", acc_id, valid_sum);
                    summary.push(acc_line);
                }
            }
            return (Status::Ok, Json(summary));
        }
        Ok(Err(e)) => {
            eprintln!("Database error during category summary retrieval: {:?}", e);
            return (Status::InternalServerError, Json(vec![]));
        }
        Err(e) => {
            eprintln!(
                "Blocking task failed during category summary retrieval: {:?}",
                e
            );
            return (Status::InternalServerError, Json(vec![]));
        }
    }
}
