// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        #[max_length = 150]
        name -> Varchar,
        description -> Nullable<Text>,
        price -> Numeric,
        stock -> Int4,
        seller_id -> Nullable<Int4>,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

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

diesel::joinable!(products -> users (seller_id));
diesel::joinable!(refresh_token -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(products, refresh_token, users,);
