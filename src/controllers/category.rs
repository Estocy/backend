use crate::models::category::{Category, NewCategory};
use crate::database::connection::establish_connection;
use crate::database::schema::categories::dsl::categories;
use uuid::Uuid;
use rocket_contrib::json::Json;
use diesel::prelude::*;


#[post("/", format="json", data = "<category>")]
pub fn create(category: Json<NewCategory>) -> Json<Category> {
    let connection = establish_connection();
    let category = diesel::insert_into(categories)
        .values(&category.0)
        .get_result(&connection)
        .unwrap();

    Json(category)
}

#[get("/")]
pub fn index() -> Json<Vec<Category>> {
    let connection = establish_connection();
    let results = categories.load::<Category>(&connection)
        .expect("Error loading posts");

    Json(results)

}

#[get("/<id>")]
pub fn show(id: String) -> Json<Category> {
    let connection = establish_connection();
    let results = categories.find(Uuid::parse_str(id.as_str()).unwrap())
        .load::<Category>(&connection)
        .expect("Error loading posts");

    Json(results[0].clone())
}

#[delete("/<id>")]
pub fn delete(id: String) -> Json<bool> {
    let connection = establish_connection();
    let client = categories.find(Uuid::parse_str(id.as_str()).unwrap());
    let result = diesel::delete(client)
        .execute(&connection);
    match result{ 
        Ok(_) => Json(true),
        Err(_) => Json(false)
    }
}