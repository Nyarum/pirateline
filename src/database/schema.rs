diesel::table! {
    accounts (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}