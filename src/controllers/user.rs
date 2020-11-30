use crate::database::connection::establish_connection;
use crate::database::schema::users::dsl;
use crate::database::schema::users::dsl::users;
use crate::models::user::{NewUser, User};
use diesel::prelude::*;
use rocket_contrib::json::Json;
use uuid::Uuid;

#[post("/", format = "json", data = "<user>")]
pub fn create(user: Json<NewUser>) -> Option<Json<User>> {
    let connection = establish_connection();

    let results = users
        .filter(dsl::email.eq(&user.0.email))
        .load::<User>(&connection)
        .expect("Error loading posts");

    if results.len() == 0 {
        let user = diesel::insert_into(users)
            .values(&user.0)
            .get_result(&connection)
            .unwrap();

        Some(Json(user))
    }
    else {
        None
    }
}

#[get("/login/<email>/<pass>")]
pub fn login(email: String, pass: String) -> Option<Json<Uuid>> {
    let connection = establish_connection();

    let results = users
        .filter(dsl::email.eq(email))
        .filter(dsl::password.eq(pass))
        .load::<User>(&connection)
        .expect("Error loading users");

    if results.len() != 0 {
        Some(Json(results[0].id))
    }
    else {
        None
    }
}

#[get("/")]
pub fn index() -> Json<Vec<User>> {
    let connection = establish_connection();

    let results = users
        .load::<User>(&connection)
        .expect("Error loading users");

    Json(results)
}

#[get("/<id>")]
pub fn show(id: String) -> Option<Json<User>> {
    let connection = establish_connection();

    let results = users
        .find(Uuid::parse_str(id.as_str()).unwrap())
        .load::<User>(&connection)
        .expect("Error loading users");

    if results.len() != 0 {
        Some(Json(results[0].clone()))
    }
    else {
        None
    }
}

#[delete("/<id>")]
pub fn delete(id: String) -> Option<()> {
    let connection = establish_connection();

    let user = users.find(Uuid::parse_str(id.as_str()).unwrap());

    let result = diesel::delete(user).execute(&connection);

    match result {
        Ok(qnt_deleted) => {
            if qnt_deleted == 0 {
                None
            }
            else {
                Some(())
            }
        }
        Err(_) => None,
    }
}
