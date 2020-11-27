#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

#[cfg(test)] mod tests;

mod controllers;
mod database;
mod models;

use rocket::{Request, Response, Data};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;

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
        client::delete,
        client::show_by_email
    ];
    let support_routes = routes![support::create];

    rocket::ignite()
        .mount("/categories", category_routes)
        .mount("/clients", client_routes)
        .mount("/products", product_routes)
        .mount("/requests", request_routes)
        .mount("/support", support_routes)
        .mount("/users", user_routes)
        .attach(CORS())
}


pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_request(&self, request: &mut Request, _data: &Data) {
        request.add_header(Header::new("Access-Control-Allow-Origin", "*"));
        request.add_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        request.add_header(Header::new("Access-Control-Allow-Headers", "*"));
        request.add_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn main() {
    let rocket = get_rocket_instance();
    rocket.launch();
}