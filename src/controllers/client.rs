use crate::models::client::{Client, NewClient};
use crate::database::connection::establish_connection;
use crate::database::schema::clients::dsl::clients;
use uuid::Uuid;
use rocket_contrib::json::Json;
use diesel::prelude::*;


#[post("/", format="json", data = "<client>")]
pub fn create(client: Json<NewClient>) -> Json<Client> {
    let connection = establish_connection();
    let client = diesel::insert_into(clients)
        .values(&client.0)
        .get_result(&connection)
        .unwrap();

    Json(client)
}

#[get("/")]
pub fn index() -> Json<Vec<Client>> {
    let mut client = Vec::new();
    client.push(Client {
        id: Uuid::new_v4(),
        email: Some(String::from("luisjuniorbr@gmail.com")),
        name: String::from("Luiz Junio"),
        address: Some(String::new()),
        phone_number: Some(String::from("31 98930-0801"))
    });
    client.push(Client {
        id: Uuid::new_v4(),
        email: Some(String::from("mihoyo@gmail.com")),
        name: String::from("Paimon"),
        address: Some(String::from("Teyvat")),
        phone_number: Some(String::from(""))
    });

    Json(client)
}

#[get("/<id>")]
pub fn show(id: String) -> Json<Client> {
    todo!()
}

#[delete("/<id>")]
pub fn delete(id: String) -> Json<bool> {
    todo!()
}