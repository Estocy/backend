use crate::models::user::{User, NewUser};
use crate::database::connection::establish_connection;
use crate::database::schema::users::dsl::users;
use uuid::Uuid;
use rocket_contrib::json::Json;
use diesel::prelude::*;


#[post("/", format="json", data = "<user>")]
pub fn create(user: Json<NewUser>) -> Json<User> {
    let connection = establish_connection();
    let user = diesel::insert_into(users)
        .values(&user.0)
        .get_result(&connection)
        .unwrap();

    Json(user)
}

#[get("/")]
pub fn index() -> Json<Vec<User>>{
    let connection = establish_connection();
    let results = users.load::<User>(&connection)
        .expect("Error loading posts");

    Json(results)
}

#[get("/<id>")]
pub fn show(id: String) -> Json<Result<User, &'static str>> {
    let connection = establish_connection();
    let results = users.find(Uuid::parse_str(id.as_str()).unwrap())
        .load::<User>(&connection)
        .expect("Error loading posts");

    if results.len() == 0{
        Json(Err("Usuário não encontrado"))
    }else{
        Json(Ok(results[0].clone()))
    }
}

#[delete("/<id>")]
pub fn delete(id: String) -> Json<bool> {
    let connection = establish_connection();
    let user = users.find(Uuid::parse_str(id.as_str()).unwrap());
    let result = diesel::delete(user)
        .execute(&connection);
    match result{ 
        Ok(_) => Json(true),
        Err(_) => Json(false)
    }
}