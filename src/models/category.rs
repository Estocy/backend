use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Category {
    id: Uuid,
    label: String,
    tag_color: String
}