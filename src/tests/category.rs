use rocket::{local::Client, http::ContentType};
use rocket::http::Status;
use serde_json;
use uuid::Uuid;

use crate::{get_rocket_instance, models::category::{Category, NewCategory}};

#[test]
fn create() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");

    let (response_category, response_category_expected) = create_category();

    assert_eq!(response_category.label, response_category_expected.label);
    assert_eq!(response_category.tag_color, response_category_expected.tag_color);

    client.delete(format!("/categories/{}",response_category.id))
        .header(ContentType::JSON)
        .dispatch();
}

#[test]
fn show() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");

    let (response_category_create, response_category_expected) = create_category();

    let mut response = client.get(format!("/categories/{}",response_category_create.id))
    .header(ContentType::JSON)
    .dispatch();

    let response_category: Category = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response_category.id, response_category_expected.id);
    assert_eq!(response_category.label, response_category_expected.label);
    assert_eq!(response_category.tag_color, response_category_expected.tag_color);

    client.delete(format!("/categories/{}",response_category.id))
        .header(ContentType::JSON)
        .dispatch();
}

#[test]
fn delete() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");

    let (response_category, response_category_expected) = create_category();

    let response = client.delete(format!("/categories/{}",response_category.id))
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}


fn create_category() -> (Category, Category){
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");
    let category = NewCategory{
        label:"teste",
        tag_color: "#ffffff"
    };

    let mut response = client.post("/categories")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&category).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_category: Category = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    let response_category_expected = Category{
        id: response_category.id,
        label: String::from("teste"),
        tag_color: String::from("#ffffff")
    };

    return (response_category, response_category_expected);
}