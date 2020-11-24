use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Client {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub address: String
}