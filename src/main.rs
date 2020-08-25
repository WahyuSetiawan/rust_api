#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate envfile;

use mysql::*;
use envfile::EnvFile;
use std::io;
use std::path::Path;

mod database;
mod services;

mod model;

mod embedded {
    use refinery::embed_migrations;

    embed_migrations!("migration");
}

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

    // let url = "mysql://root:08111993@localhost:3306/testing";

    // let pool = Pool::new(url).unwrap();

    // let mut conn = pool.get_conn().unwrap();

    // embedded::migrations::runner().run(&mut conn).unwrap();

    let mut envfile = EnvFile::new(&Path::new(".env")).unwrap();


    for (key, value) in &envfile.store {
        println!("{}: {}", key, value);
    }

    envfile.update("ID", "example");
    println!("ID: {}", envfile.get("ID").unwrap_or(""));
}
