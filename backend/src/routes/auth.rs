use rocket::serde::json::Json;
use crate::models::user::User;
use crate::handlers::auth_handler;

#[post("/signup", format = "json", data = "<user>")]
pub async fn signup(user: Json<User>) -> &'static str {
    // Delegate to a handler function for actual signup logic
    auth_handler::handle_signup(user.into_inner()).await
}
