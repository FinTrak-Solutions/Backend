use crate::db::DbPool;
use crate::models::user::User;
use crate::schema::categories::dsl::*; // For categories table
use crate::schema::transactions::dsl::*;
use crate::schema::users::dsl::{email as user_email, users}; // For users table
use diesel::dsl::sum;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

// helper struct for summary entires
#[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
pub struct SummaryEntry {
    pub nickname: String,
    pub budget: f64,
    pub budget_freq: String,
    pub transaction_date: String,
    pub amount: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Queryable, Serialize, Deserialize, Clone)]
pub struct CategorySummary {
    pub nickname: String,
    pub budget: f64,
    pub budget_freq: String,
    pub overbudget: bool,
    pub total: f64,
    // a vector of all the relevant transactions within budget freq frame
    pub cat_trans: Vec<String>,
}

// GET /report_details?email=<>
pub async fn handle_report_details(
    email_str: String,
    pool: DbPool,
) -> (Status, Json<Vec<CategorySummary>>) {
    // Step 1: validate email
    let (email_status, _user_email) = check_email_valid(email_str.clone(), pool.clone()).await;

    if email_status != Status::Ok {
        return (Status::BadRequest, Json(vec![]));
    }

    // Step 2: get summary of category
    let category_result = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_search = email_str.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            // order matters: https://stackoverflow.com/questions/72670161/how-do-you-use-rust-diesel-to-do-a-group-by-query
            // nullable types: https://docs.diesel.rs/1.4.x/diesel/sql_types/struct.Nullable.html
            transactions
                .inner_join(categories)
                .filter(crate::schema::transactions::dsl::email.eq(email_to_search))
                .select((
                    crate::schema::categories::dsl::nickname,
                    crate::schema::categories::dsl::budget,
                    crate::schema::categories::dsl::budget_freq,
                    crate::schema::transactions::dsl::transaction_date,
                    crate::schema::transactions::dsl::amount,
                    crate::schema::transactions::dsl::notes,
                ))
                .order_by((
                    crate::schema::transactions::dsl::category_id,
                    crate::schema::transactions::dsl::transaction_date,
                ))
                .load::<SummaryEntry>(&mut conn)
        }
    })
    .await;

    match category_result {
        Ok(Ok(trans_list)) => {
            // Declare summary vector
            let mut summary: Vec<CategorySummary> = vec![];
            let mut cat_summary_dict: HashMap<String, CategorySummary> = HashMap::new();
            // Successfully retrieved category transactions
            for sum_entry in trans_list.into_iter() {
                let mut curr_cat_sum: CategorySummary =
                    match cat_summary_dict.contains_key(&sum_entry.nickname) {
                        true => match cat_summary_dict.get(&sum_entry.nickname) {
                            Some(existing_cat) => existing_cat.clone(),
                            None => {
                                return (Status::InternalServerError, Json(vec![]));
                            }
                        },
                        false => {
                            let new_cat = CategorySummary {
                                nickname: sum_entry.nickname.clone(),
                                budget: sum_entry.budget.clone(),
                                budget_freq: sum_entry.budget_freq.clone(),
                                overbudget: false,
                                total: 0.0,
                                cat_trans: vec![],
                            };
                            cat_summary_dict.insert(sum_entry.nickname.clone(), new_cat.clone());
                            new_cat
                        }
                    };
                // check if entry is within the budget frame:
                let budget_freq_str = curr_cat_sum.budget_freq.as_str();
                let budget_timeframe = match budget_freq_str {
                    "weekly" => 7.0 * 24.0 * 60.0,
                    "monthly" => 30.0 * 24.0 * 60.0,
                    "yearly" => 365.0 * 24.0 * 60.0,
                    _ => f64::INFINITY,
                };
                let dt_str = sum_entry.transaction_date.as_str();
                let txn_datetime: chrono::DateTime<chrono::Utc> = dt_str.parse().unwrap();
                let diff = chrono::Utc::now() - txn_datetime;
                if diff.num_minutes() > budget_timeframe.round() as i64 {
                    // outside of the budget range concern
                    continue;
                }
                // include the amount of the current entry, and check budget status
                curr_cat_sum.total += sum_entry.amount;
                if curr_cat_sum.total > curr_cat_sum.budget {
                    curr_cat_sum.overbudget = true;
                }
                let curr_line = match sum_entry.notes {
                    Some(valid_notes) => format!(
                        "{}, {}, {}",
                        sum_entry.transaction_date, sum_entry.amount, valid_notes
                    ),
                    None => format!("{}, {}, ", sum_entry.transaction_date, sum_entry.amount,),
                };
                curr_cat_sum.cat_trans.push(curr_line);
                // refresh the summary copy
                cat_summary_dict.insert(sum_entry.nickname.clone(), curr_cat_sum.clone());
            }
            // populate summary
            for (_, sum_obj) in &cat_summary_dict {
                summary.push(sum_obj.clone());
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
