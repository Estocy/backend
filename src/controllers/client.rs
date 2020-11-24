use crate::models::client::Client;
use rocket_contrib::json::Json;
use uuid::Uuid;

#[post("/", format="json", data = "<client>")]
pub fn create(client: Json<Client>) -> Json<Client> {
    todo!()
}

#[get("/")]
pub fn index() -> Json<Vec<Client>> {
    let mut clients = Vec::new();
    clients.push(Client {
        id: Uuid::new_v4(),
        email: String::from("luisjuniorbr@gmail.com"),
        name: String::from("Luiz Junio"),
        address: String::new(),
        phone_number: String::from("31 98930-0801")
    });
    clients.push(Client {
        id: Uuid::new_v4(),
        email: String::from("mihoyo@gmail.com"),
        name: String::from("Paimon"),
        address: String::from("Teyvat"),
        phone_number: String::from("")
    });

    Json(clients)
}

#[get("/<id>")]
pub fn show(id: String) -> Json<Client> {
    todo!()
}

#[delete("/<id>")]
pub fn delete(id: String) -> Json<bool> {
    todo!()
}