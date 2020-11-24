use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum RequestStatus {
   Open,
   Completed,
   InProgress,
   Canceled
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Request {
    id: Uuid,
    user_id: Uuid,
    client_id: Uuid,
    sale_date: DateTime::<Utc>,
    delivery_date: DateTime::<Utc>,
    status: RequestStatus,
    comments: String,
    price: f32,
    discount: f32,
    freight: f32
}