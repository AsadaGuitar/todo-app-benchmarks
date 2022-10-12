use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
use crate::schema::user_accounts;
use crate::schema::tasks;

#[derive(Debug, Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "user_accounts"]
pub struct UserAccounts {
    pub id: String,
    pub close: bool,
    pub create_at: DateTime<Utc>,
    pub close_at: Option<DateTime<Utc>>
}   

#[derive(Debug, Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct Tasks {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub close: bool,
    pub create_at: DateTime<Utc>,
    pub modify_at: Option<DateTime<Utc>>,
    pub close_at: Option<DateTime<Utc>>
}