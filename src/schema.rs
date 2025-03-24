// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        phone_number -> Nullable<Text>,
        email -> Nullable<Text>,
    }
}
