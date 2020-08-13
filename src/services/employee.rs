#![feature(proc_macro_hygiene, decl_macro)]

#[get("/employee")]
pub fn get() -> &'static str {
    return "get all employee";
}

#[post("/employee")]
pub fn set() -> &'static str {
    "set employee"
}

pub fn get_all() -> &'static str {
    "get all employee"
}
