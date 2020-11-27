use crate::models::user::{User, NewUser, Login};
use crate::database::connection::establish_connection;
use crate::database::schema::users::dsl;
use crate::database::schema::users::dsl::users;
use uuid::Uuid;
use rocket_contrib::json::Json;
use diesel::prelude::*;


#[post("/", format="json", data = "<user>")]
pub fn create(user: Json<NewUser>) -> Json<User> {
    println!("{:?}", user);
    let connection = establish_connection();
    let user = diesel::insert_into(users)
        .values(&user.0)
        .get_result(&connection)
        .unwrap();

    Json(user)
}

#[get("/login", format="json", data = "<user>")]
pub fn login(user: Json<Login>) -> Option<()> {
    let connection = establish_connection();
    let results = users.filter(dsl::email.eq(&user.0.email))
        .filter(dsl::password.eq(&user.0.password))
        .load::<User>(&connection)
        .expect("Error loading posts");

    if results.len() == 0 {
        None
    } else {
        Some(())
    }
}

#[get("/")]
pub fn index() -> Json<Vec<User>>{
    let connection = establish_connection();
    let results = users.load::<User>(&connection)
        .expect("Error loading posts");

    Json(results)
}

#[get("/<id>")]
pub fn show(id: String) -> Option<Json<User>> {
    let connection = establish_connection();
    let results = users.find(Uuid::parse_str(id.as_str()).unwrap())
        .load::<User>(&connection)
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
    let user = users.find(Uuid::parse_str(id.as_str()).unwrap());
    let result = diesel::delete(user)
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