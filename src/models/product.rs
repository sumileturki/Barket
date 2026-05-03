use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use bigdecimal::BigDecimal;

use chrono::NaiveDateTime;

#[derive(Serialize, Queryable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
    pub seller_id: Option<i32>,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize,Insertable)]
#[diesel(table_name = crate::schema::products)]
pub struct NewProduct{
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
    pub seller_id: Option<i32>,
}