#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate envfile;
#[macro_use]
extern crate mysql;

mod database;
mod env;
mod services;

mod model;

fn main() {
    // rocket::ignite()
    //     .mount(
    //         "/",
    //         routes![
    //             services::public::index,
    //             services::employee::getSingle,
    //             services::employee::get
    //         ],
    //     )
    //     .launch();
}
