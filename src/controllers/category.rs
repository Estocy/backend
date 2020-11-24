use crate::models::category::Category;
use crate::database::connection::establish_connection;
use uuid::Uuid;
use rocket_contrib::json::Json;


#[post("/create", format="json", data = "<category>")]
pub fn create(category: Json<Category>) {
    let connection = establish_connection();
    todo!()
}

#[get("/index")]
pub fn index() -> Json<Vec<Category>> {
    let connection = establish_connection();
    todo!()
}

#[get("/<id>/show")]
pub fn show(id: String) -> Json<Category> {
    let connection = establish_connection();
    todo!()

}

#[delete("/<id>/delete")]
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