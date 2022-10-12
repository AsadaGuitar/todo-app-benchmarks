use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
use crate::models::Tasks;

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectAllResponse {
    pub id: String,
    pub user_id: String,
    pub tasks: Vec<Tasks>,
    pub create_at: DateTime<Utc>,
    pub time_zone: String
}