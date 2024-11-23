#[allow(unused_imports)]
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}
