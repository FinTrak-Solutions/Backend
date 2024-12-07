// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (account_id) {
        account_id -> Int4,
        email -> Text,
        account_type -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
        username -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    users,
);
