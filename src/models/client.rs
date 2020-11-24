use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Client {
    id: Uuid,
    name: String,
    email: String,
    phone_number: String,
    address: String
}