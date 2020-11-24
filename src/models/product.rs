use uuid::Uuid;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Product {
    id: Uuid,
    name: String,
    description: String,
    store_name: String,
    store_price: f32,
    price: f32,
    additional_charge: f32,
    color: String,
    weight: f32,
    brand: String,
    stock_amount: u32
}