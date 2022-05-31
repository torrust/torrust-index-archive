use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invite {
    pub invite_id: i64,
    pub valid: bool,
    pub username: Option<String>,
    pub key: String,
}
