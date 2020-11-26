use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::database::schema::requests_products;


#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct RequestProduct{
    pub id: Uuid,
    pub request_id: Uuid,
    pub product_id: Uuid,
    pub amount: i32,
    pub additional_costs: f32,
    pub discount: f32
}

#[derive(Insertable, Deserialize, Serialize, Clone, Debug)]
#[table_name="requests_products"]
pub struct NewRequestProduct {
    pub request_id: Uuid,
    pub product_id: Uuid,
    pub amount: i32,
    pub additional_costs: f32,
    pub discount: f32
}