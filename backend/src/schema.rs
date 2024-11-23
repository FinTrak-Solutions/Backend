// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
        username -> Text,
    }
}
