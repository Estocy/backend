use serde::{Serialize, Deserialize};
use diesel::Queryable;

#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct Support {
    pub message: String
}