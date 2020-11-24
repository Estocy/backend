use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::Queryable;

#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct User {
    id: Uuid,
    name: String,
    email: String,
    password: String,
    share_photos: bool,
    darkmode: bool
}