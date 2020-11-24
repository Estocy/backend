#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod controllers;
mod database;
mod models;

fn main() {
    let user_routes = routes![];
    let request_routes = routes![];
    let product_routes = routes![];
    let category_routes = routes![];
    let client_routes = routes![];
    let support_routes = routes![];
    rocket::ignite().mount("/user", user_routes).launch();
    rocket::ignite().mount("/request", request_routes).launch();
    rocket::ignite().mount("/product", product_routes).launch();
    rocket::ignite().mount("/category", category_routes).launch();
    rocket::ignite().mount("/client", client_routes).launch();
    rocket::ignite().mount("/support", support_routes).launch();
}