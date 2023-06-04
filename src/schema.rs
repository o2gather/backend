// @generated automatically by Diesel CLI.

diesel::table! {
    event_comments (id) {
        id -> Uuid,
        event_id -> Uuid,
        user_id -> Uuid,
        content -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    event_members (event_id, user_id) {
        event_id -> Uuid,
        user_id -> Uuid,
        amount -> Int8,
    }
}

diesel::table! {
    events (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        category -> Text,
        start_time -> Timestamp,
        end_time -> Timestamp,
        min_amount -> Int8,
        max_amount -> Int8,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        phone -> Varchar,
        avatar -> Text,
        guid -> Text,
    }
}

diesel::joinable!(event_comments -> events (event_id));
diesel::joinable!(event_comments -> users (user_id));
diesel::joinable!(event_members -> events (event_id));
diesel::joinable!(event_members -> users (user_id));
diesel::joinable!(events -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    event_comments,
    event_members,
    events,
    users,
);
