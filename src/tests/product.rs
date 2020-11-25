use rocket::{local::Client, http::ContentType};
use rocket::http::Status;
use serde_json;
use uuid::Uuid;
use rocket_contrib::json::Json;

use crate::{get_rocket_instance, models::product::{Product, NewProduct}};

#[test]
fn create() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");
    let product = NewProduct{
        name: "Produto",
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

    let response_product_expected = Product{
            id: Uuid::new_v4(),
            name: String::from("Produto"),
            description: String::from("Descricao"),
            store_name: Some(String::from("Store name")),
            store_price: Some(10.5),
            price: 10.5,
            additional_charge: Some(10.0),
            color: String::from("#ffffff"),
            weight: 10.0,
            brand: String::from("brand"),
            stock_amount: 10
    };

    let mut response = client.post("/products")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&product).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_product: Product = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response_product.name, response_product_expected.name);
    assert_eq!(response_product.description, response_product_expected.description);
    assert_eq!(response_product.store_name, response_product_expected.store_name);
    assert_eq!(response_product.store_price, response_product_expected.store_price);
    assert_eq!(response_product.price, response_product_expected.price);
    assert_eq!(response_product.additional_charge, response_product_expected.additional_charge);
    assert_eq!(response_product.color, response_product_expected.color);
    assert_eq!(response_product.weight, response_product_expected.weight);
    assert_eq!(response_product.brand, response_product_expected.brand);
    assert_eq!(response_product.stock_amount, response_product_expected.stock_amount);

    let mut response = client.delete(format!("/products/{}",response_product.id))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&product).unwrap())
        .dispatch();
}

#[test]
fn index() {
    todo!()
}

#[test]
fn show() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");
    let product = NewProduct{
        name: "Produto",
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

    let mut response = client.post("/products")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&product).unwrap())
        .dispatch();

    let response_product_create: Product = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    let response_product_expected = Product{
        id: response_product_create.id,
        name: String::from("Produto"),
        description: String::from("Descricao"),
        store_name: Some(String::from("Store name")),
        store_price: Some(10.5),
        price: 10.5,
        additional_charge: Some(10.0),
        color: String::from("#ffffff"),
        weight: 10.0,
        brand: String::from("brand"),
        stock_amount: 10
    };

    let mut response = client.get(format!("/products/{}",response_product_create.id))
    .header(ContentType::JSON)
    .body(serde_json::to_string(&product).unwrap())
    .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_product: Product = serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response_product.id, response_product_expected.id);
    assert_eq!(response_product.name, response_product_expected.name);
    assert_eq!(response_product.description, response_product_expected.description);
    assert_eq!(response_product.store_name, response_product_expected.store_name);
    assert_eq!(response_product.store_price, response_product_expected.store_price);
    assert_eq!(response_product.price, response_product_expected.price);
    assert_eq!(response_product.additional_charge, response_product_expected.additional_charge);
    assert_eq!(response_product.color, response_product_expected.color);
    assert_eq!(response_product.weight, response_product_expected.weight);
    assert_eq!(response_product.brand, response_product_expected.brand);
    assert_eq!(response_product.stock_amount, response_product_expected.stock_amount);

    let mut response = client.delete(format!("/products/{}",response_product_create.id))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&product).unwrap())
        .dispatch();
}

#[test]
fn delete() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");

    let categories = NewProduct{
        name: "Produto",
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
    let mut response_create = client.post("/products")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&categories).unwrap())
        .dispatch();
    let response_product_create: Product = serde_json::from_str(response_create.body_string().unwrap().as_str()).unwrap();

    let mut response = client.delete(format!("/products/{}",response_product_create.id))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&categories).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}
