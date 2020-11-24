use crate::models::product::Product;
use uuid::Uuid;
use rocket_contrib::json::Json;

#[post("/", format="json", data = "<product>")]
pub fn create(product: Json<Product>) -> Json<Product> {
    todo!()
}

#[get("/")]
pub fn index() -> Json<Vec<Product>>{
    todo!()
}

#[get("/<id>")]
pub fn show(id: String) -> Json<Product> {
    todo!()
}

#[delete("/<id>")]
pub fn delete(id: String) -> Json<bool> {
    todo!()
}