// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        external_id -> Uuid,
        sku -> Varchar,
        name -> Varchar,
        description -> Varchar,
        price -> Int8,
    }
}
