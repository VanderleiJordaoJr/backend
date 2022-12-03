use diesel::prelude::*;

#[derive(Queryable, serde::Serialize)]
pub struct Product {
    pub id: i32,
    pub external_id: uuid::Uuid,
    pub sku: String,
    pub name: String,
    pub description: String,
    pub price: i64,
}
