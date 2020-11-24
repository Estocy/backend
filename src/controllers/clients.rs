use crate::models::client::Client;
use rocket_contrib::json::Json;
use uuid::Uuid;

#[post("/create", format="json", data = "<client>")]
pub fn create(client: Json<Client>) -> Json<Client> {
    todo!()
}

#[get("/index")]
pub fn index() -> Json<Vec<Client>> {
    todo!()
}

#[get("/<id>/show")]
pub fn show(id: String) -> Json<Client> {
    todo!()
}

#[delete("/<id>/delete")]
pub fn delete(id: String) -> Json<bool> {
    todo!()
}