use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackerKey {
    pub key: String,
    pub valid_until: i64,
}
