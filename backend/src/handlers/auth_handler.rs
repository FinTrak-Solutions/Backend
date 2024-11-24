use crate::models::user::NewUser;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
#[allow(unused_imports)]
use diesel::result::Error;
use crate::db::DbPool;

pub async fn handle_signup(user: crate::models::user::NewUser, pool: DbPool) -> &'static str {
    // Add business logic here, e.g., validate input, hash password, and save to database
    if user.email.is_empty() || user.password.is_empty() || user.username.is_empty() {
        return "Invalid input";
    }
    println!("Signup request received for user: {}", user.username);
    println!("Signup request received for email: {}", user.email);

    // create new user instance
    let new_user = NewUser {
        email: user.email.clone(),
        password: user.password.clone(),
        username: user.username.clone(),
    };

    // Insert the new user into the database
    let result = tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().expect("Failed to get database connection");
        diesel::insert_into(users)
            .values(&new_user)
            .execute(&mut conn)
    })
        .await;

    match result {
        Ok(Ok(_)) => "User successfully registered",
        Ok(Err(e)) => {
            eprintln!("Database error: {:?}", e);
            "Failed to register user"
        }
        Err(_) => "Internal server error",
    }

    // "User successfully registered"
}
