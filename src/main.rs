#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod controllers;

fn main() {
    let routes = routes![];
    rocket::ignite().mount("/", routes).launch();
}