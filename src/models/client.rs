use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::Queryable;

#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct Client {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub address: String
}
