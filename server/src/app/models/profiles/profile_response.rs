use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProfileEntity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub age: i32,
    pub about_me: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
