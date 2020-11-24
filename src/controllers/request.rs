use rocket_contrib::json::Json;
use uuid::Uuid;

use crate::models::request::Request;

#[post("/", format="json", data = "<request>")]
pub fn create(request: Json<Request>) -> Json<Request> {

    todo!()
}

#[get("/")]
pub fn index() -> Json<Vec<Request>> {
    todo!()

}

#[get("/<id>")]
pub fn show(id: String) -> Json<Request>{
    todo!()

}

#[delete("/<id>")]
pub fn delete(id: String) -> Json<Request>{
    todo!()

}