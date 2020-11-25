use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};

use crate::database::schema::clients;

#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct Client {
    pub id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>
}

#[derive(Insertable, Deserialize, Serialize, Clone, Debug)]
#[table_name="clients"]
pub struct NewClient<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub phone_number: &'a str,
    pub address: &'a str
}
