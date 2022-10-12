// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Varchar,
        user_id -> Varchar,
        title -> Varchar,
        close -> Bool,
        create_at -> Timestamptz,
        modify_at -> Nullable<Timestamptz>,
        close_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    user_accounts (id) {
        id -> Varchar,
        close -> Bool,
        create_at -> Timestamptz,
        close_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(tasks -> user_accounts (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    user_accounts,
);
