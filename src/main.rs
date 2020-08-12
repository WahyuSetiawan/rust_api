#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod database;
mod services;

mod model;

fn main() {
    let data_employee = model::Employee::getAll();

    println!("{:?}",data_employee.len());
}
