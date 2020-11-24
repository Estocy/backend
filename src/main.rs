#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

#[cfg(test)] mod tests;

mod controllers;
mod database;
mod models;


use controllers::{
    category,
    client,
    product,
    request,
    support,
    user
};

fn get_rocket_instance() -> rocket::Rocket {
    let user_routes = routes![
        user::create,
        user::index,
        user::show,
        user::delete
    ];
    let request_routes = routes![
        request::create,
        request::index,
        request::show,
        request::delete
    ];
    let product_routes = routes![
        product::create,
        product::index,
        product::show,
        product::delete
    ];
    let category_routes = routes![
        category::create,
        category::index,
        category::show,
        category::delete
    ];
    let client_routes = routes![
        client::create,
        client::index,
        client::show,
        client::delete
    ];
    let support_routes = routes![];

    rocket::ignite()
        .mount("/users", user_routes)
        .mount("/requests", request_routes)
        .mount("/products", product_routes)
        .mount("/categories", category_routes)
        .mount("/clients", client_routes)
        .mount("/support", support_routes)
}

fn main() {
    let rocket = get_rocket_instance();
    rocket.launch();
}