#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod database;
mod services;

fn main() {
    // rocket::ignite().mount("/", routes![services::index, hello, other::world,])
    // .launch();
    database::testing_insert_data();
}
