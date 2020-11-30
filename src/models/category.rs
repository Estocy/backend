use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::schema::categories;

#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct Category {
    pub id: Uuid,
    pub label: String,
    pub tag_color: String,
}

#[derive(Insertable, Deserialize, Serialize, Clone, Debug)]
#[table_name = "categories"]
pub struct NewCategory<'a> {
    pub label: &'a str,
    pub tag_color: &'a str,
}
