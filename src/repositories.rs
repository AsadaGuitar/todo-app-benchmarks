use chrono::Utc;
use diesel::prelude::*;
use diesel::result;
use crate::models::{UserAccounts, Tasks};
use crate::schema::{user_accounts, tasks};

pub fn insert_user_account(conn: &mut PgConnection, user_account: UserAccounts) -> Result<UserAccounts, result::Error> {
    diesel::insert_into(user_accounts::table)
        .values(user_account)
        .get_result::<UserAccounts>(conn)
}

pub fn insert_task(conn: &mut PgConnection, task: Tasks) -> Result<Tasks, result::Error> {
    diesel::insert_into(tasks::table)
        .values(task)
        .get_result(conn)
}

pub fn update_task(conn: &mut PgConnection, task: Tasks) -> Result<Tasks, result::Error> {
    diesel::update(
        tasks::dsl::tasks
            .filter(tasks::id.eq(task.id))
            .filter(tasks::user_id.eq(task.user_id))
            .filter(tasks::close.eq(false))
    )
        .set((tasks::title.eq(task.title), tasks::modify_at.eq(task.modify_at)))
        .get_result(conn)
}

pub fn find_task_by_id(conn: &mut PgConnection, user_id: &str, task_id: &str) -> Result<Option<Tasks>, result::Error> {
    tasks::dsl::tasks
        .filter(tasks::user_id.eq(user_id))
        .filter(tasks::close.eq(false))
        .find(task_id)
        .select((tasks::id, tasks::user_id, tasks::title, tasks::close, tasks::create_at, tasks::modify_at, tasks::close_at))
        .get_result(conn)
        .optional()
}

pub fn find_all_tasks_by_user_id(conn: &mut PgConnection, user_id: &str) -> Result<Vec<Tasks>, result::Error> {
    tasks::dsl::tasks
        .filter(tasks::user_id.eq(user_id))
        .filter(tasks::close.eq(false))
        .load::<Tasks>(conn)
}

pub fn delete_task_by_id(conn: &mut PgConnection, user_id: &str, task_id: &str) -> Result<Tasks, result::Error> {
    diesel::update(
        tasks::dsl::tasks
            .filter(tasks::id.eq(task_id))
            .filter(tasks::user_id.eq(user_id))
            .filter(tasks::close.eq(false))
    )
        .set((tasks::close.eq(true), tasks::close_at.eq(Some(Utc::now()))))
        .get_result(conn)
}