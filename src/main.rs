#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod controllers;
mod database;
mod models;
#[cfg(test)] mod tests;

use controllers::client;

fn get_rocket_instance() -> rocket::Rocket {
    let user_routes = routes![];
    let request_routes = routes![];
    let product_routes = routes![];
    let category_routes = routes![];
    let client_routes = routes![client::index];
    let support_routes = routes![];

    rocket::ignite().mount("/user", user_routes)
                    .mount("/request", request_routes)
                    .mount("/product", product_routes)
                    .mount("/category", category_routes)
                    .mount("/client", client_routes)
                    .mount("/support", support_routes)
}

fn main() {
    let rocket = get_rocket_instance();
    rocket.launch();
}