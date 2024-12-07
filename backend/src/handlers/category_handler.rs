use crate::db::DbPool;
use crate::models::category::{Category, NewCategory};
use crate::models::user::User;
use crate::schema::categories::dsl::*;
use crate::schema::users::dsl::{email as user_email, users}; // For users table
use diesel::prelude::*;
use rocket::http::Status;

pub async fn handle_category_create(new_cat: NewCategory, pool: DbPool) -> (Status, String) {
    // Step 1: Validate input
    if new_cat.email.is_empty() || new_cat.category_type.is_empty() || new_cat.nickname.is_empty() {
        return (Status::BadRequest, "Invalid input".to_string());
    }

    // Step 1.5: Check if the email exists in users table
    // If not, no account should be created
    let user_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_check = new_cat.email.clone();
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
            // User found, proceed to account name existence check
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

    // Step 2: Check if the category nickname already exists for the given email
    let nickname_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_check = new_cat.email.clone();
        let name_to_check = new_cat.nickname.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            categories
                .filter(email.eq(email_to_check))
                .filter(nickname.eq(name_to_check))
                .first::<Category>(&mut conn)
                .optional()
        }
    })
    .await;

    match nickname_exists {
        Ok(Ok(Some(_existing_nickname))) => {
            // Category nickname already taken for this email
            return (
                Status::BadRequest,
                "Failed to create new category: duplicate nicknames".to_string(),
            );
        }
        Ok(Ok(None)) => {
            // Step 3: Proceed to create the new category
            let cat_to_insert = new_cat.clone(); // Clone the NewCategory to avoid moving it
            let nickname_for_message = new_cat.clone(); // Another clone to access nickname later
            let result = tokio::task::spawn_blocking({
                let pool = pool.clone();
                move || {
                    let mut conn = pool.get().expect("Failed to get database connection");
                    diesel::insert_into(categories)
                        .values(&cat_to_insert) // Use one copy of the NewCategory
                        .execute(&mut conn)
                }
            })
            .await;

            match result {
                Ok(Ok(_)) => {
                    // Successfully inserted the account
                    let msg = format!("Successfully created {}", nickname_for_message.nickname);
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
        Ok(Err(e)) => {
            eprintln!("Error checking account existence: {:?}", e);
            (Status::InternalServerError, "Database error".to_string())
        }
        Err(e) => {
            eprintln!("Blocking task failed during account check: {:?}", e);
            (
                Status::InternalServerError,
                "Internal server error".to_string(),
            )
        }
    }
}
