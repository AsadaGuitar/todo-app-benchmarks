table! {
    tasks (id) {
        id -> Varchar,
        user_id -> Varchar,
        title -> Varchar,
        details -> Nullable<Text>,
        close -> Bool,
        create_at -> Timestamptz,
        modify_at -> Nullable<Timestamptz>,
        close_at -> Nullable<Timestamptz>,
    }
}

table! {
    user_accounts (id) {
        id -> Varchar,
        name -> Varchar,
        password -> Varchar,
        close -> Bool,
        create_at -> Timestamptz,
        modify_at -> Nullable<Timestamptz>,
        close_at -> Nullable<Timestamptz>,
    }
}

joinable!(tasks -> user_accounts (user_id));

allow_tables_to_appear_in_same_query!(
    tasks,
    user_accounts,
);
