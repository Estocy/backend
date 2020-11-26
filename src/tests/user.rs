use rocket::{local::Client, http::ContentType};
use rocket::http::Status;
use serde_json;
use uuid::Uuid;

use crate::{get_rocket_instance, models::user::{User, NewUser}};

#[test]
fn create() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");

    let (response_user, response_user_expected) = create_user();

    assert_eq!(response_user.name, response_user_expected.name);
    assert_eq!(response_user.email, response_user_expected.email);
    assert_eq!(response_user.password, response_user_expected.password);
    assert_eq!(response_user.share_photos, response_user_expected.share_photos);
    assert_eq!(response_user.darkmode, response_user_expected.darkmode);

    client.delete(format!("/users/{}",response_user.id))
        .header(ContentType::JSON)
        .dispatch();
}

#[test]
fn show() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");

    let (response_user_create, response_user_expected) = create_user();

    let mut response = client.get(format!("/users/{}",response_user_create.id))
        .header(ContentType::JSON)
        .dispatch();

    let response_user: User = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response_user.id, response_user_expected.id);
    assert_eq!(response_user.name, response_user_expected.name);
    assert_eq!(response_user.email, response_user_expected.email);
    assert_eq!(response_user.password, response_user_expected.password);
    assert_eq!(response_user.share_photos, response_user_expected.share_photos);
    assert_eq!(response_user.darkmode, response_user_expected.darkmode);

    client.delete(format!("/users/{}",response_user_create.id))
        .header(ContentType::JSON)
        .dispatch();
}

#[test]
fn delete() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");

    let (response_user_create, response_user_expected) = create_user();

    let response = client.delete(format!("/users/{}",response_user_create.id))
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

fn create_user() -> (User, User){
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");

    let users = NewUser{
        name: "Quick",
        email: "henrique.fquick@gmail.com",
        password: "123",
        share_photos: None,
        darkmode: Some(false)
    };

    let mut response_create = client.post("/users")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&users).unwrap())
        .dispatch();
    let response_user_create: User = serde_json::from_str(response_create.body_string().unwrap().as_str()).unwrap();

    let response_user_expected = User{
        id: response_user_create.id,
        name: String::from("Quick"),
        email: String::from("henrique.fquick@gmail.com"),
        password: String::from("123"),
        share_photos: Some(false),
        darkmode: Some(false)
    };

    return (response_user_create, response_user_expected);
}