use crate::database::connection::establish_connection;
use crate::database::schema::products::dsl::products;
use crate::models::product::{Product, NewProduct};
use diesel::prelude::*;
use rocket_contrib::json::Json;
use uuid::Uuid;

#[post("/", format="json", data = "<product>")]
pub fn create(product: Json<NewProduct>) -> Json<Product> {
    let connection = establish_connection();
    let product = diesel::insert_into(products)
        .values(&product.0)
        .get_result(&connection)
        .unwrap();

    Json(product)
}

#[get("/")]
pub fn index() -> Json<Vec<Product>> {
    let connection = establish_connection();
    let results = products.load::<Product>(&connection)
        .expect("Error loading posts");

    Json(results)
}

#[get("/<id>")]
pub fn show(id: String) -> Json<Product> {
    let connection = establish_connection();
    let results = products.find(Uuid::parse_str(id.as_str()).unwrap())
        .load::<Product>(&connection)
        .expect("Error loading posts");

    Json(results[0].clone())
}

#[delete("/<id>")]
pub fn delete(id: String) -> Json<bool> {
    let connection = establish_connection();
    let product = products.find(Uuid::parse_str(id.as_str()).unwrap());
    let result = diesel::delete(product)
        .execute(&connection);

    match result{
        Ok(_) => Json(true),
        Err(_) => Json(false)
    }
}