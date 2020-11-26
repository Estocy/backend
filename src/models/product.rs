use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::database::schema::products;

#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub code: i32,
    pub description: String,
    pub store_name: Option<String>,
    pub store_price: Option<f32>,
    pub price: f32,
    pub additional_charge: Option<f32>,
    pub color: String,
    pub weight: f32,
    pub brand: String,
    pub stock_amount: i32
}


#[derive(Insertable, Deserialize, Serialize, Clone, Debug)]
#[table_name="products"]
pub struct NewProduct<'a> {
    pub name: &'a str,
    pub code: i32,
    pub description: &'a str,
    pub store_name: Option<&'a str>,
    pub store_price: Option<f32>,
    pub price: f32,
    pub additional_charge: Option<f32>,
    pub color: &'a str,
    pub weight: f32,
    pub brand: &'a str,
    pub stock_amount: i32
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProductReceiver<'a>{
    #[serde(borrow)]
    pub product: NewProduct<'a>,
    pub categories: Vec<Uuid>
}