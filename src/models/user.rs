use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};

use crate::database::schema::users;

#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub share_photos: Option<bool>,
    pub darkmode: Option<bool>
}

#[derive(Insertable, Deserialize, Serialize, Clone, Debug)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub share_photos: Option<bool>,
    pub darkmode: Option<bool>
}
