use crate::database::schema::requests_products;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct RequestProduct {
    pub id: Uuid,
    pub request_id: Uuid,
    pub product_id: Uuid,
    pub amount: i32,
    pub additional_costs: f32,
    pub discount: f32,
}

#[derive(Insertable, Deserialize, Serialize, Clone, Debug)]
#[table_name = "requests_products"]
pub struct NewRequestProduct {
    pub request_id: Uuid,
    pub product_id: Uuid,
    pub amount: i32,
    pub additional_costs: f32,
    pub discount: f32,
}
