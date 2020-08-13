#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod database;
mod services;

mod model;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![services::public::index, services::employee::get],
        )
        .launch();
}
