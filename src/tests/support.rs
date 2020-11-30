use rocket::http::Status;
use rocket::{http::ContentType, local::Client};
use serde_json;

use crate::{get_rocket_instance, models::support::Support};

#[test]
fn create() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");

    let response_support_expected = Support {
        message: String::from("teste"),
    };

    let mut response = client
        .post("/support")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&response_support_expected).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_support: Support =
        serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response_support.message, response_support_expected.message);
}
