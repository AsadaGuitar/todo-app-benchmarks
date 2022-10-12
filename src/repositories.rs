use diesel::prelude::*;
use crate::models::UserAccounts;
use crate::schema::user_accounts;

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

