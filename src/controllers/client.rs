use crate::models::client::{Client, NewClient};
use crate::database::connection::establish_connection;
use crate::database::schema::clients::dsl;
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

#[get("/<id_user>")]
pub fn index(id_user: String) -> Json<Vec<Client>> {
    let connection = establish_connection();
    let results = clients.filter(dsl::user_id.eq(Uuid::parse_str(id_user.as_str()).unwrap()))
        .load::<Client>(&connection)
        .expect("Error loading posts");

    Json(results)
}

#[get("/<id_user>/<id>")]
pub fn show(id_user: String, id: String) -> Option<Json<Client>> {
    let connection = establish_connection();
    let results = clients.filter(dsl::id.eq(Uuid::parse_str(id.as_str()).unwrap()))
        .filter(dsl::user_id.eq(Uuid::parse_str(id_user.as_str()).unwrap()))
        .load::<Client>(&connection)
        .expect("Error loading posts");

    if results.len() == 0 {
        None
    } else {
        Some(Json(results[0].clone()))
    }
}

#[delete("/<id_user>/<id>")]
pub fn delete(id_user: String, id: String) -> Option<()> {
    let connection = establish_connection();
    let client = clients.filter(dsl::id.eq(Uuid::parse_str(id.as_str()).unwrap()))
        .filter(dsl::user_id.eq(Uuid::parse_str(id_user.as_str()).unwrap()));

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