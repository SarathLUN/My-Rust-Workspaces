// @generated automatically by Diesel CLI.

diesel::table! {
    articles (id) {
        id -> Uuid,
        title -> Varchar,
        content -> Text,
        published_at -> Timestamptz,
        is_published -> Bool,
        is_deleted -> Bool,
        deleted_at -> Nullable<Timestamptz>,
    }
}
