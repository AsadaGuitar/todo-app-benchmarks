use diesel::prelude::*;
use crate::models::{UserAccounts, Tasks};
use crate::schema::{user_accounts, tasks};

pub fn insert_user_account(conn: &mut PgConnection, user_account: UserAccounts) -> UserAccounts {
    diesel::insert_into(user_accounts::table)
        .values(user_account)
        .get_result(conn)
        .expect("Error saving new user_account.")
}

pub fn insert_task(conn: &mut PgConnection, task: Tasks) -> Tasks {
    diesel::insert_into(tasks::table)
        .values(task)
        .get_result(conn)
        .expect("Error saving new task.")
}

pub fn update_task(conn: &mut PgConnection, task: Tasks) -> Tasks {
    diesel::update(
        tasks::dsl::tasks
            .filter(tasks::id.eq(task.id))
            .filter(tasks::user_id.eq(task.user_id)))        
            .set((tasks::title.eq(task.title), tasks::modify_at.eq(task.modify_at)))
            .get_result(conn)
            .expect("Error udpating tasks")
}

pub fn find_task_by_id(conn: &mut PgConnection, user_id: &str, task_id: &str) -> Option<Tasks> {
    tasks::dsl::tasks
        .filter(tasks::user_id.eq(user_id))
        .filter(tasks::close.eq(false))
        .find(task_id)
        .select((tasks::id, tasks::user_id, tasks::title, tasks::close, tasks::create_at, tasks::modify_at, tasks::close_at))
        .get_result(conn)
        .optional()
        .expect("Error loading tasks")
}

pub fn find_all_tasks_by_user_id(conn: &mut PgConnection, user_id: &str) -> Vec<Tasks> {
    tasks::dsl::tasks
        .filter(tasks::user_id.eq(user_id))
        .filter(tasks::close.eq(false))
        .load::<Tasks>(conn)
        .expect("Error loading tasks")
}

pub fn delete_task_by_id(conn: &mut PgConnection, user_id: &str, task_id: &str) -> usize {
    diesel::delete(
        tasks::dsl::tasks
            .filter(tasks::id.eq(task_id))
            .filter(tasks::user_id.eq(user_id))
    ).execute(conn).expect("Error deleting tasks.")
}