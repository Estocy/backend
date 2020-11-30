use chrono::NaiveDate;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::schema::requests;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum RequestStatus {
    Open,
    Completed,
    InProgress,
    Canceled,
}

#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct Request {
    pub id: Uuid,
    pub code: i32,
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub sale_date: NaiveDate,
    pub delivery_date: NaiveDate,
    pub status: i32,
    pub comments: Option<String>,
    pub price: f32,
    pub discount: f32,
    pub freight: f32,
}

#[derive(Insertable, Deserialize, Serialize, Clone, Debug)]
#[table_name = "requests"]
pub struct NewRequest<'a> {
    pub code: i32,
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub sale_date: NaiveDate,
    pub delivery_date: NaiveDate,
    pub status: i32,
    pub comments: Option<&'a str>,
    pub price: f32,
    pub discount: f32,
    pub freight: f32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RequestReceiver<'a> {
    #[serde(borrow)]
    pub request: NewRequest<'a>,
    pub product_id: Uuid,
    pub amount: i32,
    pub additional_costs: f32,
    pub discount: f32,
}
