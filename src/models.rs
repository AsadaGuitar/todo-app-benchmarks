use crate::schema::user_accounts;
use crate::schema::tasks;
use chrono::{Utc, DateTime};

#[derive(Debug, Insertable, Queryable)]
#[table_name = "user_accounts"]
pub struct UserAccounts {
    pub id: String,
    pub name: String,
    pub password: String,
    pub close: bool,
    pub create_at: DateTime<Utc>,
    pub modify_at: Option<DateTime<Utc>>,
    pub close_at: Option<DateTime<Utc>>
}   

#[derive(Insertable, Queryable)]
#[table_name = "tasks"]
pub struct Tasks {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub details: String,
    pub close: bool,
    pub create_at: DateTime<Utc>,
    pub modify_at: Option<DateTime<Utc>>,
    pub close_at: Option<DateTime<Utc>>
}