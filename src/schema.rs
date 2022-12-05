// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Text,
        title -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        completed -> Nullable<Varchar>,
    }
}
