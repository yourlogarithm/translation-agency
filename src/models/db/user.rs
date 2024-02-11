use chrono::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub verification_code: String,
    pub verified: bool,
    pub creation_timestamp: Option<DateTime<Utc>>,
    pub update_timestamp: Option<DateTime<Utc>>
}
