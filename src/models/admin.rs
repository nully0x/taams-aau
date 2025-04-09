use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Admin {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
}
