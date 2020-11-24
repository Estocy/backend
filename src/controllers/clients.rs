use crate::models::client::Client;
use rocket_contrib::json::Json;
use uuid::Uuid;

#[post("/create", format="json", data = "<client>")]
pub fn create(client: Json<Client>) -> Json<Client> {
    todo!()
}

#[get("/index")]
pub fn index() -> Json<Vec<Client>> {
    let mut clients = Vec::new();
    clients.push(Client {
        id: Uuid::new_v4(),
        email: String::from("luisjuniorbr@gmail.com"),
        name: String::from("Luiz Junio"),
        address: String::new(),
        phone_number: String::from("31 98930-0801")
    });

    Json(clients)
}

#[get("/<id>/show")]
pub fn show(id: String) -> Json<Client> {
    todo!()
}

#[delete("/<id>/delete")]
pub fn delete(id: String) -> Json<bool> {
    todo!()
}