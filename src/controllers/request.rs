use rocket_contrib::json::Json;
use uuid::Uuid;

use crate::models::request::Request;

#[post("/create", format="json", data = "<request>")]
pub fn create(request: Json<Request>) -> Json<Request> {

    todo!()
}

#[get("/index")]
pub fn index() -> Json<Vec<Request>> {
    todo!()

}

#[get("/<id>/create")]
pub fn show(id: String) -> Json<Request>{
    todo!()

}

#[delete("/<id>/delete")]
pub fn delete(id: String) -> Json<Request>{
    todo!()

}