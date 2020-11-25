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
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");

    let categories = NewCategory{
        label: "test",
        tag_color: "#ffffff",
    };
    let mut response_create = client.post("/categories")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&categories).unwrap())
        .dispatch();
    let response_category_create: Category = serde_json::from_str(response_create.body_string().unwrap().as_str()).unwrap();

    let response_category_expected = Category{
            id: response_category_create.id,
            label: String::from("test"),
            tag_color: String::from("#ffffff")
    };

    let mut response = client.get(format!("/categories/{}",response_category_create.id))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&categories).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_category: Category = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response_category.id, response_category_expected.id);
    assert_eq!(response_category.label, response_category_expected.label);
    assert_eq!(response_category.tag_color, response_category_expected.tag_color);

    let mut response = client.delete(format!("/categories/{}",response_category_create.id))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&categories).unwrap())
        .dispatch();
}

#[test]
fn delete() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");

    let categories = NewCategory{
        label: "test",
        tag_color: "#ffffff",
    };
    let mut response_create = client.post("/categories")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&categories).unwrap())
        .dispatch();
    let response_category_create: Category = serde_json::from_str(response_create.body_string().unwrap().as_str()).unwrap();

    let mut response = client.delete(format!("/categories/{}",response_category_create.id))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&categories).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string().unwrap(), "true");
}
