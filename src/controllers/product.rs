use crate::database::connection::establish_connection;
use crate::database::schema::products::dsl::products;
use crate::database::schema::products_categories::dsl;
use crate::database::schema::products_categories::dsl::products_categories;
use crate::models::product::{Product, NewProduct, ProductReceiver};
use crate::models::product_category::{NewProductCategory, ProductCategory};
use diesel::prelude::*;
use rocket_contrib::json::Json;
use uuid::Uuid;

#[post("/", format="json", data = "<product>")]
pub fn create(product: Json<ProductReceiver>) -> Json<Product> {
    let connection = establish_connection();

    let product_created: Product = diesel::insert_into(products)
        .values(&product.0.product)
        .get_result(&connection)
        .unwrap();

    for category in &product.0.categories {
        let product_category:ProductCategory = diesel::insert_into(products_categories)
            .values( (dsl::product_id.eq(product_created.id), dsl::category_id.eq(category)))
            .get_result(&connection)
            .unwrap();
    }

    Json(product_created)
}

#[get("/")]
pub fn index() -> Json<Vec<Product>> {
    let connection = establish_connection();
    let results = products.load::<Product>(&connection)
        .expect("Error loading posts");

    Json(results)
}

#[get("/<id>")]
pub fn show(id: String) -> Option<Json<Product>> {
    let connection = establish_connection();
    let results = products.find(Uuid::parse_str(id.as_str()).unwrap())
        .load::<Product>(&connection)
        .expect("Error loading posts");

    if results.len() == 0 {
        None
    } else {
        Some(Json(results[0].clone()))
    }
}

#[delete("/<id>")]
pub fn delete(id: String) -> Option<()> {
    let connection = establish_connection();
    let product = products.find(Uuid::parse_str(id.as_str()).unwrap());
    let result = diesel::delete(product)
        .execute(&connection);

    match result {
        Ok(qnt_deleted) => {
            if qnt_deleted == 0 {
                None
            } else {
                Some(())
            }
        },
        Err(_) => None
    }
}