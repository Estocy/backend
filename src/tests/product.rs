use rocket::http::Status;
use rocket::{http::ContentType, local::Client};
use rocket_contrib::json::Json;
use serde_json;
use uuid::Uuid;

use crate::{
    get_rocket_instance,
    models::category::{Category, NewCategory},
    models::product::{NewProduct, Product, ProductReceiver},
};

#[test]
fn create() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");

    let product = NewProduct {
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
        stock_amount: 10,
    };

    let category = NewCategory {
        label: "teste",
        tag_color: "#ffffff",
    };

    let mut response = client
        .post("/categories")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&category).unwrap())
        .dispatch();

    let response_category: Category =
        serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    let mut categories = Vec::<Uuid>::new();

    categories.push(response_category.id);

    let product_category = ProductReceiver {
        product: product,
        categories: categories,
    };

    let response_product_expected = Product {
        id: Uuid::new_v4(),
        name: String::from("Produto"),
        code: 1,
        description: String::from("Descricao"),
        store_name: Some(String::from("Store name")),
        store_price: Some(10.5),
        price: 10.5,
        additional_charge: Some(10.0),
        color: String::from("#ffffff"),
        weight: 10.0,
        brand: String::from("brand"),
        stock_amount: 10,
    };

    let mut response = client
        .post("/products")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&product_category).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_product: Product =
        serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response_product.name, response_product_expected.name);

    assert_eq!(response_product.code, response_product_expected.code);

    assert_eq!(
        response_product.description,
        response_product_expected.description
    );

    assert_eq!(
        response_product.store_name,
        response_product_expected.store_name
    );

    assert_eq!(
        response_product.store_price,
        response_product_expected.store_price
    );

    assert_eq!(response_product.price, response_product_expected.price);

    assert_eq!(
        response_product.additional_charge,
        response_product_expected.additional_charge
    );

    assert_eq!(response_product.color, response_product_expected.color);

    assert_eq!(response_product.weight, response_product_expected.weight);

    assert_eq!(response_product.brand, response_product_expected.brand);

    assert_eq!(
        response_product.stock_amount,
        response_product_expected.stock_amount
    );

    let mut response = client
        .delete(format!("/products/{}", response_product.id))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&product_category).unwrap())
        .dispatch();
}

#[test]
fn show() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");

    let product = NewProduct {
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
        stock_amount: 10,
    };

    let expected_product = Product {
        id: Uuid::new_v4(),
        name: String::from("Produto"),
        code: 1,
        description: String::from("Descricao"),
        store_name: Some(String::from("Store name")),
        store_price: Some(10.5),
        price: 10.5,
        additional_charge: Some(10.0),
        color: String::from("#ffffff"),
        weight: 10.0,
        brand: String::from("brand"),
        stock_amount: 10,
    };

    let category = NewCategory {
        label: "teste",
        tag_color: "#ffffff",
    };

    let mut response = client
        .post("/categories")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&category).unwrap())
        .dispatch();

    let response_category: Category =
        serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    let mut categories = Vec::<Uuid>::new();

    categories.push(response_category.id);

    let product_category = ProductReceiver {
        product: product,
        categories: categories,
    };

    let mut response_create = client
        .post("/products")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&product_category).unwrap())
        .dispatch();

    let response_create_product: Product =
        serde_json::from_str(response_create.body_string().unwrap().as_str()).unwrap();

    let mut response = client
        .get(format!("/products/{}", response_create_product.id))
        .header(ContentType::JSON)
        .dispatch();

    let response_product: (Product, Vec<Category>) =
        serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response_product.0.name, expected_product.name);

    assert_eq!(response_product.0.code, expected_product.code);

    assert_eq!(response_product.0.description, expected_product.description);

    assert_eq!(response_product.0.store_name, expected_product.store_name);

    assert_eq!(response_product.0.store_price, expected_product.store_price);

    assert_eq!(response_product.0.price, expected_product.price);

    assert_eq!(
        response_product.0.additional_charge,
        expected_product.additional_charge
    );

    assert_eq!(response_product.0.color, expected_product.color);

    assert_eq!(response_product.0.weight, expected_product.weight);

    assert_eq!(response_product.0.brand, expected_product.brand);

    assert_eq!(
        response_product.0.stock_amount,
        expected_product.stock_amount
    );

    assert_eq!(response_product.1[0].id, response_category.id);

    assert_eq!(response_product.1[0].label, response_category.label);

    assert_eq!(response_product.1[0].tag_color, response_category.tag_color);

    let mut response = client
        .delete(format!("/products/{}", response_product.0.id))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&product_category).unwrap())
        .dispatch();
}

#[test]
fn delete() {
    let client = Client::new(get_rocket_instance()).expect("valid rocket instance");

    let product = NewProduct {
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
        stock_amount: 10,
    };

    let category = NewCategory {
        label: "teste",
        tag_color: "#ffffff",
    };

    let mut response = client
        .post("/categories")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&category).unwrap())
        .dispatch();

    let response_category: Category =
        serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    let mut categories = Vec::<Uuid>::new();

    categories.push(response_category.id);

    let product_category = ProductReceiver {
        product: product,
        categories: categories,
    };

    let response_product_expected = Product {
        id: Uuid::new_v4(),
        name: String::from("Produto"),
        code: 1,
        description: String::from("Descricao"),
        store_name: Some(String::from("Store name")),
        store_price: Some(10.5),
        price: 10.5,
        additional_charge: Some(10.0),
        color: String::from("#ffffff"),
        weight: 10.0,
        brand: String::from("brand"),
        stock_amount: 10,
    };

    let mut response = client
        .post("/products")
        .header(ContentType::JSON)
        .body(serde_json::to_string(&product_category).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let response_product: Product =
        serde_json::from_str(response.body_string().unwrap().as_str()).unwrap();

    assert_eq!(response_product.name, response_product_expected.name);

    assert_eq!(response_product.code, response_product_expected.code);

    assert_eq!(
        response_product.description,
        response_product_expected.description
    );

    assert_eq!(
        response_product.store_name,
        response_product_expected.store_name
    );

    assert_eq!(
        response_product.store_price,
        response_product_expected.store_price
    );

    assert_eq!(response_product.price, response_product_expected.price);

    assert_eq!(
        response_product.additional_charge,
        response_product_expected.additional_charge
    );

    assert_eq!(response_product.color, response_product_expected.color);

    assert_eq!(response_product.weight, response_product_expected.weight);

    assert_eq!(response_product.brand, response_product_expected.brand);

    assert_eq!(
        response_product.stock_amount,
        response_product_expected.stock_amount
    );

    let response = client
        .delete(format!("/products/{}", response_product.id))
        .header(ContentType::JSON)
        .body(serde_json::to_string(&product_category).unwrap())
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}
