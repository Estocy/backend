use rocket::local::Client;
use rocket::http::Status;

use crate::get_rocket_instance;

#[test]
fn create() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");
    //let mut response = client.get("/").dispatch();
    //assert_eq!(response.status(), Status::Ok);
    //assert_eq!(response.body_string(), Some("Hello, world!".into()));
    todo!();
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
