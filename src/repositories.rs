use diesel::prelude::*;
use crate::models::{UserAccounts, Tasks};
use crate::schema::{user_accounts, tasks};

pub fn insert_user_account(conn: &mut PgConnection, user_account: UserAccounts) -> UserAccounts {
    diesel::insert_into(user_accounts::table)
        .values(user_account)
        .get_result(conn)
        .expect("Error saving new user_account.")
}

pub fn find_all_user_accounts(conn: &mut PgConnection) -> Vec<UserAccounts> {
    user_accounts::dsl::user_accounts
        .load::<UserAccounts>(conn)
        .expect("Error loading user_accounts")
}

pub fn find_all_tasks_by_user_id(conn: &mut PgConnection, user_id: &str) -> Vec<Tasks> {
    tasks::dsl::tasks
        .filter(tasks::user_id.eq(user_id))
        .load::<Tasks>(conn)
        .expect("Error loading tasks")
}