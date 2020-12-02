#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[cfg(test)]
mod tests;

mod controllers;
mod database;
mod models;

use rocket_cors::CorsOptions;

use controllers::{category, client, product, request, support, user};

fn get_rocket_instance() -> rocket::Rocket {
    let user_routes = routes![
        user::create,
        user::index,
        user::show,
        user::delete,
        user::login
    ];

    let request_routes = routes![
        request::create,
        request::index,
        request::show,
        request::delete
    ];

    let product_routes = routes![
        product::create,
        product::update,
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
        client::update,
        client::index,
        client::show,
        client::delete,
        client::show_by_email
    ];

    let support_routes = routes![support::create];

    let cors = CorsOptions::default().to_cors().unwrap();

    rocket::ignite()
        .mount("/categories", category_routes)
        .mount("/clients", client_routes)
        .mount("/products", product_routes)
        .mount("/requests", request_routes)
        .mount("/support", support_routes)
        .mount("/users", user_routes)
        .attach(cors)
}

fn main() {
    let rocket = get_rocket_instance();

    rocket.launch();
}
