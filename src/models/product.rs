use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::database::schema::products;

#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct Product {
    id: Uuid,
    name: String,
    description: String,
    store_name: Option<String>,
    store_price: Option<f32>,
    price: f32,
    additional_charge: Option<f32>,
    color: String,
    weight: f32,
    brand: String,
    stock_amount: i32
}


#[derive(Insertable, Deserialize, Serialize, Clone, Debug)]
#[table_name="products"]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub store_name: &'a str,
    pub store_price: Option<f32>,
    pub price: f32,
    pub additional_charge: Option<f32>,
    pub color: &'a str,
    pub weight: f32,
    pub brand: &'a str,
    pub stock_amount: i32
}