// @generated automatically by Diesel CLI.

diesel::table! {
    refresh_token (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        token -> Text,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        phone -> Nullable<Varchar>,
        password -> Text,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(refresh_token -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(refresh_token, users,);
