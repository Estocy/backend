use uuid::Uuid;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::database::schema::products_categories;


#[derive(Queryable, Deserialize, Serialize, Clone, Debug)]
pub struct ProductCategory{
    pub id: Uuid,
    pub product_id: Uuid,
    pub category_id: Uuid
}

#[derive(Insertable, Deserialize, Serialize, Clone, Debug)]
#[table_name="products_categories"]
pub struct NewProductCategory {
    pub product_id: Uuid,
    pub category_id: Uuid
}