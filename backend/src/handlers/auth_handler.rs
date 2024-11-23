use crate::models::user::User;

pub async fn handle_signup(user: User) -> &'static str {
    // Add business logic here, e.g., validate input, hash password, and save to database
    if user.email.is_empty() || user.password.is_empty() || user.username.is_empty() {
        return "Invalid input";
    }
    println!("Signup request received for user: {}", user.username);
    println!("Signup request received for email: {}", user.email);
    // Example placeholder response
    "User successfully registered"
}
