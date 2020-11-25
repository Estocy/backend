use rocket::http::ContentType;
use rocket::http::Status;
use serde_json;
use uuid::Uuid;
use rocket_contrib::json::Json;

use crate::{get_rocket_instance, models::client::{Client, NewClient}};

#[test]
fn create() {
    let client = rocket::local::Client::new(get_rocket_instance()).expect("valid rocket instance");
    let clients = NewClient{
        name: "Quick2",
        email: "henrique.fquick@gmail.com",
        phone_number: "31998180608",
        address: "Rua teste"
    };

    let response_client_expected = Client{
            id: Uuid::new_v4(),
            name: String::from("Quick2"),
            email: Some(String::from("henrique.fquick@gmail.com")),
            phone_number: Some(String::from("31998180608")),
            address: Some(String::from("Rua teste"))
    };

    let mut response = client.post("/clients")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&clients).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_client: Client = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response_client.name, response_client_expected.name);
    assert_eq!(response_client.email, response_client_expected.email);
    assert_eq!(response_client.phone_number, response_client_expected.phone_number);
    assert_eq!(response_client.address, response_client_expected.address);

    let mut response = client.delete(format!("/clients/{}",response_client.id))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&clients).unwrap())
        .dispatch();
}

#[test]
fn index() {
    todo!()
}

#[test]
fn show() {
    let client = rocket::local::Client::new(get_rocket_instance()).expect("valid rocket instance");

    let clients = NewClient{
        name: "Quick",
        email: "henrique.fquick@gmail.com",
        phone_number: "31998180608",
        address: "Rua teste"
    };
    let mut response_create = client.post("/clients")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&clients).unwrap())
        .dispatch();
    let response_client_create: Client = serde_json::from_str(response_create.body_string().unwrap().as_str()).unwrap();

    let response_client_expected = Client{
            id: response_client_create.id,
            name: String::from("Quick"),
            email: Some(String::from("henrique.fquick@gmail.com")),
            phone_number: Some(String::from("31998180608")),
            address: Some(String::from("Rua teste"))
    };

    let mut response = client.get(format!("/clients/{}",response_client_create.id))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&clients).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_client: Client = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response_client.id, response_client_expected.id);
    assert_eq!(response_client.name, response_client_expected.name);
    assert_eq!(response_client.email, response_client_expected.email);
    assert_eq!(response_client.phone_number, response_client_expected.phone_number);
    assert_eq!(response_client.address, response_client_expected.address);

    let mut response = client.delete(format!("/clients/{}",response_client_create.id))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&clients).unwrap())
        .dispatch();
}

#[test]
fn delete() {
    let client = rocket::local::Client::new(get_rocket_instance()).expect("valid rocket instance");

    let clients = NewClient{
        name: "Quick",
        email: "henrique.fquick@gmail.com",
        phone_number: "31998180608",
        address: "Rua teste"
    };
    let mut response_create = client.post("/clients")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&clients).unwrap())
        .dispatch();
    let response_client_create: Client = serde_json::from_str(response_create.body_string().unwrap().as_str()).unwrap();

    let response_client_expected = Client{
            id: response_client_create.id,
            name: String::from("Quick"),
            email: Some(String::from("henrique.fquick@gmail.com")),
            phone_number: Some(String::from("31998180608")),
            address: Some(String::from("Rua teste"))
    };

    let mut response = client.delete(format!("/clients/{}",response_client_create.id))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&clients).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}
