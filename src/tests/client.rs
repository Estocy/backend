use rocket::http::ContentType;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_json;
use uuid::Uuid;

use crate::{
    get_rocket_instance,
    models::client::{Client, NewClient},
    models::user::{NewUser, User},
};

#[test]
fn create() {
    let client = rocket::local::Client::new(get_rocket_instance()).expect("valid rocket instance");

    let (response_client, response_client_expected, user_id) = create_client();

    assert_eq!(response_client.name, response_client_expected.name);

    assert_eq!(response_client.email, response_client_expected.email);

    assert_eq!(
        response_client.phone_number,
        response_client_expected.phone_number
    );

    assert_eq!(response_client.address, response_client_expected.address);

    client
        .delete(format!("/clients/{}/{}", user_id, response_client.id))
        .header(ContentType::JSON)
        .dispatch();
}

#[test]
fn show() {
    let client = rocket::local::Client::new(get_rocket_instance()).expect("valid rocket instance");

    let (response_client_create, response_client_expected, user_id) = create_client();

    let mut response = client
        .get(format!(
            "/clients/{}/{}",
            user_id, response_client_create.id
        ))
        .header(ContentType::JSON)
        .dispatch();

    let response_client: Client =
        serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response_client.id, response_client_expected.id);

    assert_eq!(response_client.name, response_client_expected.name);

    assert_eq!(response_client.email, response_client_expected.email);

    assert_eq!(
        response_client.phone_number,
        response_client_expected.phone_number
    );

    assert_eq!(response_client.address, response_client_expected.address);

    client
        .delete(format!("/clients/{}/{}", user_id, response_client.id))
        .header(ContentType::JSON)
        .dispatch();
}

#[test]
fn delete() {
    let client = rocket::local::Client::new(get_rocket_instance()).expect("valid rocket instance");

    let (response_client, response_client_expected, user_id) = create_client();

    let response = client
        .delete(format!("/clients/{}/{}", user_id, response_client.id))
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

fn create_client() -> (Client, Client, Uuid) {
    let client = rocket::local::Client::new(get_rocket_instance()).expect("valid rocket instance");

    let users = NewUser {
        name: "Quick",
        email: "henrique.fquick@gmail.com",
        password: "123",
        share_photos: None,
        darkmode: Some(false),
    };

    let mut response_create = client
        .post("/users")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&users).unwrap())
        .dispatch();

    let response_user_create: User =
        serde_json::from_str(response_create.body_string().unwrap().as_str()).unwrap();

    let clients = NewClient {
        user_id: response_user_create.id,
        name: "Quick",
        email: "henrique.fquick@gmail.com",
        phone_number: "31998180608",
        address: "Rua teste",
    };

    let mut response_create = client
        .post("/clients")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&clients).unwrap())
        .dispatch();

    let response_client_create: Client =
        serde_json::from_str(response_create.body_string().unwrap().as_str()).unwrap();

    let response_client_expected = Client {
        id: response_client_create.id,
        user_id: response_user_create.id,
        name: String::from("Quick"),
        email: Some(String::from("henrique.fquick@gmail.com")),
        phone_number: Some(String::from("31998180608")),
        address: Some(String::from("Rua teste")),
    };

    let mut response = client
        .get(format!(
            "/clients/{}/{}",
            response_user_create.id, response_client_create.id
        ))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&clients).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_client: Client =
        serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    return (
        response_client,
        response_client_expected,
        response_user_create.id,
    );
}
