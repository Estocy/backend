use crate::models::category::{Category, NewCategory};
use crate::database::connection::establish_connection;
use crate::database::schema::categories::dsl::*;
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
    todo!()
}

#[get("/<id>")]
pub fn show(id: String) -> Json<Category> {
    let connection = establish_connection();
    todo!()

}

#[delete("/<id>")]
pub fn delete(id: String) -> Json<bool> {
    let connection = establish_connection();
    todo!()
}
/*
use self::models::{Category, NewCategory};

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}
 */