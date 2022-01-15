use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub email_verified: bool,
    pub password: String,
    pub administrator: bool,
    pub banned: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // username
    pub admin: bool,
    pub exp: u64, // epoch in seconds
}
