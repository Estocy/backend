use crate::models::user::User;
use uuid::Uuid;

#[post("/create")]
pub fn create(user: User) -> User {
    todo!()
}

#[delete("/<id>/delete")]
pub fn delete(id: Uuid) -> bool{
    todo!()
}