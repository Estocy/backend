use crate::models::user::User;
use rocket_contrib::json::Json;
use uuid::Uuid;

#[post("/create", format="json", data = "<user>")]
pub fn create(user: Json<User>) -> Json<User> {
    todo!()
}

#[delete("/<id>/delete")]
pub fn delete(id: String) -> Json<bool> {
    todo!()
}