use rocket::http::ContentType;
use rocket::http::Status;
use serde_json;
use uuid::Uuid;
use rocket_contrib::json::Json;
use chrono::NaiveDate;

use crate::{
    get_rocket_instance, 
    models::product::{Product, NewProduct, ProductReceiver}, 
    models::request::{Request, NewRequest, RequestReceiver},
    models::user::{User, NewUser},
    models::client::{Client, NewClient}};

#[test]
fn create() {
    let client = rocket::local::Client::new(get_rocket_instance()).expect("valid rocket instance");

    let (response_request, response_request_expected) = createRequest();

    assert_eq!(response_request.user_id, response_request_expected.user_id);
    assert_eq!(response_request.client_id, response_request_expected.client_id);
    assert_eq!(response_request.sale_date, response_request_expected.sale_date);
    assert_eq!(response_request.delivery_date, response_request_expected.delivery_date);
    assert_eq!(response_request.status, response_request_expected.status);
    assert_eq!(response_request.comments, response_request_expected.comments);
    assert_eq!(response_request.price, response_request_expected.price);
    assert_eq!(response_request.discount, response_request_expected.discount);
    assert_eq!(response_request.freight, response_request_expected.freight);

    client.delete(format!("/requests/{}",response_request.id))
    .header(ContentType::JSON)
    .dispatch();

}

#[test]
fn show() {
    let client = rocket::local::Client::new(get_rocket_instance()).expect("valid rocket instance");
    let (response_request, response_request_expected) = createRequest();

    let mut response = client.get(format!("/requests/{}",response_request.id))
    .header(ContentType::JSON)
    .dispatch();

    let response_request: Request = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response_request.user_id, response_request_expected.user_id);
    assert_eq!(response_request.client_id, response_request_expected.client_id);
    assert_eq!(response_request.sale_date, response_request_expected.sale_date);
    assert_eq!(response_request.delivery_date, response_request_expected.delivery_date);
    assert_eq!(response_request.status, response_request_expected.status);
    assert_eq!(response_request.comments, response_request_expected.comments);
    assert_eq!(response_request.price, response_request_expected.price);
    assert_eq!(response_request.discount, response_request_expected.discount);
    assert_eq!(response_request.freight, response_request_expected.freight);

    client.delete(format!("/requests/{}",response_request.id))
    .header(ContentType::JSON)
    .dispatch();
}

#[test]
fn delete() {
    let client = rocket::local::Client::new(get_rocket_instance()).expect("valid rocket instance");
    let (response_request, response_request_expected) = createRequest();

    let mut response = client.delete(format!("/requests/{}",response_request.id))
        .header(ContentType::JSON)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}



fn createRequest() -> (Request, Request){
    let client = rocket::local::Client::new(get_rocket_instance()).expect("valid rocket instance");

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

    let clients = NewClient{
        user_id:response_user_create.id,
        name: "Quick2",
        email: "henrique.fquick@gmail.com",
        phone_number: "31998180608",
        address: "Rua teste"
    };
    let mut response_client = client.post(format!("/clients"))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&clients).unwrap())
        .dispatch();
    let response_client_created: Client = serde_json::from_str(response_client.body_string().unwrap().as_str()).unwrap();


    let product = NewProduct{
        name: "Produto",
        code: 1,
        description: "Descricao",
        store_name: Some("Store name"),
        store_price: Some(10.5),
        price: 10.5,
        additional_charge: Some(10.0),
        color: "#ffffff",
        weight: 10.0,
        brand: "brand",
        stock_amount: 10
    };
    let product_category = ProductReceiver{
        product:product,
        categories:Vec::<Uuid>::new()
    };
    let mut response_product = client.post("/products")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&product_category).unwrap())
        .dispatch();

    let response_product_created: Product = serde_json::from_str(response_product.body_string().unwrap().as_str()).unwrap();


    let request = NewRequest{
        code: 1,
        user_id: response_user_create.id,
        client_id: response_client_created.id,
        sale_date: NaiveDate::from_ymd(2020, 11, 25),
        delivery_date: NaiveDate::from_ymd(2020, 11, 25),
        status: 0,
        comments: Some("comentário"),
        price: 10.0,
        discount: 0.0,
        freight: 5.0
    };



    let request_product = RequestReceiver{
        request: request,
        product_id:response_product_created.id,
        amount: 1,
        additional_costs: 10.5,
        discount: 2.0
    };

    let response_request_expected = Request{
        id: Uuid::new_v4(),
        code: 1,
        user_id: response_user_create.id,
        client_id: response_client_created.id,
        sale_date: NaiveDate::from_ymd(2020, 11, 25),
        delivery_date: NaiveDate::from_ymd(2020, 11, 25),
        status: 0,
        comments: Some(String::from("comentário")),
        price: 10.0,
        discount: 0.0,
        freight: 5.0
    };

    let mut response = client.post("/requests")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&request_product).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_request: Request = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    return (response_request, response_request_expected);
}
