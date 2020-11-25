use crate::models::client::{Client, NewClient};
use crate::database::connection::establish_connection;
use crate::database::schema::clients::dsl::clients;
use uuid::Uuid;
use rocket_contrib::json::Json;
use diesel::{deserialize::QueryableByName, prelude::*};


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
    let connection = establish_connection();
    let results = clients.load::<Client>(&connection)
        .expect("Error loading posts");

    Json(results)
}

#[get("/<id>")]
pub fn show(id: String) -> Option<Json<Client>> {
    let connection = establish_connection();
    let results = clients.find(Uuid::parse_str(id.as_str()).unwrap())
        .load::<Client>(&connection)
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
    let client = clients.find(Uuid::parse_str(id.as_str()).unwrap());

    let result = diesel::delete(client)
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