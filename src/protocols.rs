use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};
use crate::models::Tasks;

#[derive(Serialize, Deserialize, Debug)]
pub struct FindAllTasksResponseJson {
    pub id: String,
    pub user_id: String,
    pub tasks: Vec<Tasks>,
    pub create_at: DateTime<Utc>,
    pub time_zone: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FindTaskByIdResponseJson {
    pub id: String,
    pub user_id: String,
    pub task: Tasks,
    pub create_at: DateTime<Utc>,
    pub time_zone: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostTaskRequestJson {
    pub title: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostTaskResponseJson {
    pub id: String,
    pub task_id: String,
    pub user_id: String,
    pub title: String,
    pub create_at: DateTime<Utc>,
    pub time_zone: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskRequestJson {
    pub title: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTaskResponseJson {
    pub id: String,
    pub task: Tasks,
    pub create_at: DateTime<Utc>,
    pub time_zone: String
}
