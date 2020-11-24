use crate::models::product::Product;
use uuid::Uuid;
use rocket_contrib::json::Json;

#[post("/create", format="json", data = "<product>")]
pub fn create(product: Json<Product>) -> Json<Product> {
    todo!()
}

#[get("/index")]
pub fn index() -> Json<Vec<Product>>{
    todo!()
}

#[get("/<id>/show")]
pub fn show(id: String) -> Json<Product> {
    todo!()
}

#[delete("/<id>/delete")]
pub fn delete(id: String) -> Json<bool> {
    todo!()
}