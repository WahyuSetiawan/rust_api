#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, outside world!"
}

mod other{
    #[get("/world")]
    pub fn world() -> &'static str{
        "hello, world!"
    }
}

fn main() {
    rocket::ignite().mount("/hello", routes![hello, other::world]).launch();
}