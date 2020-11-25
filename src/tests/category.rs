use rocket::{local::Client, http::ContentType};
use rocket::http::Status;
use serde_json;
use uuid::Uuid;

use crate::{get_rocket_instance, models::category::{Category, NewCategory}};

#[test]
fn create() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");
    let category = NewCategory{
        label:"teste",
        tag_color: "#ffffff"
    };

    let response_category_expected = Category{
            id: Uuid::new_v4(),
            label: String::from("teste"),
            tag_color: String::from("#ffffff")
    };

    let mut response = client.post("/categories")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&category).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_category: Category = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response_category.label, response_category_expected.label);
    assert_eq!(response_category.tag_color, response_category_expected.tag_color);
}

#[test]
fn index() {
    todo!()
}

#[test]
fn show() {
    todo!()
}

#[test]
fn delete() {
    todo!()
}
