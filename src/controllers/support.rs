use crate::models::support::Support;
use rocket_contrib::json::Json;

#[post("/create", format="json", data = "<support>")]
pub fn create(support: Json<Support>) -> Json<bool> {
    todo!()
}