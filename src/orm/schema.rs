// @generated automatically by Diesel CLI.

diesel::table! {
    message (id) {
        id -> Bpchar,
        text -> Text,
        answer_id -> Nullable<Bpchar>,
        created_at -> Timestamptz,
    }
}
