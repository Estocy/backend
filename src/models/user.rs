use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    id: Uuid,
    name: String,
    email: String,
    password: String,
    share_photos: bool,
    darkmode: bool
}