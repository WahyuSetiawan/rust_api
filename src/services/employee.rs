use rocket::http::RawStr;

use crate::model;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    status: i32,
    data: Option<model::Employee>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadGetAllData {
    status: i32,
    data: Vec<model::Employee>,
}

#[get("/employee")]
pub fn get() -> Json<LoadGetAllData> {
    let many_data_employee = model::Employee::getAll();

    return Json(LoadGetAllData {
        status: 200,
        data: many_data_employee,
    });
}

#[post("/employee")]
pub fn set() -> &'static str {
    "set employee"
}

#[get("/employee/<id>")]
pub fn getSingle(id: &RawStr) -> Json<Message> {
    match id.url_decode() {
        Ok(id_employee) => {
            print!("{:?}", id_employee);

            let id: i32 = id_employee.parse().expect("msg");

            let employee_single = model::Employee::get(id);

            return Json(Message {
                status: 200,
                data: employee_single,
            });
        }
        _ => println!("cannot lanjut progress"),
    }

    return Json(Message {
        status: 404,
        data: None,
    });
}
