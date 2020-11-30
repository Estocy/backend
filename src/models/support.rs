use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct Support {
    pub message: String,
}
