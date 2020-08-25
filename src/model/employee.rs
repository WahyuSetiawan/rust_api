use crate::database;
use serde::Serialize;

use mysql::prelude::*;
// use mysql::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub people_id: i32,
    pub name: String,
}

impl Employee {
    pub fn update(&self) -> Result<bool, String> {
        let mut connection_database = database::connectionDatabase().unwrap();

        let result = connection_database.as_mut().exec_drop(
            "update employee set name = :name where id = :id",
            params! {
                "id" => self.people_id,
                "name" => &self.name,
            },
        );

        if let Ok(res) = result {
            return Ok(true);
        } else {
            return Err("a commat".to_string());
        }
    }

    pub fn insert(&self) -> Result<bool, String> {
        let connection = database::connectionDatabase();
        if let Ok(conn) = connection {
            let status_insert = conn.as_mut().exec_drop(
                "insert into employee (id, name) values (:id, :name)",
                params! {"id" =>self.people_id, "name" => &self.name},
            );

            if let Ok(status) = status_insert {
                return Ok(true);
            } else {
                println!("{:?}", status_insert);

                return Err("cannot to insert data".to_string());
            }
        } else {
            return Err(format!(
                "cant connect into database sqlite {:?}",
                connection
            ));
        }
    }

    // pub fn getAll() -> Vec<Employee> {
    //     let mut all_employee: Vec<Employee> = Vec::new();
    //     let connection = database::connectionDatabase();

    //     if let Ok(conn) = connection {
    //         // let mut statement_get_all_employee =
    //         //     conn.query_map("select id, name from employee").unwrap();

    //         // all_employee = statement_get_all_employee
    //         //     .query_map(params![], |row| {
    //         //         Ok(Employee {
    //         //             people_id: row.get(0).expect("not column for id 1"),
    //         //             name: row.get(1).expect("not column for id 2"),
    //         //         })
    //         //     })
    //         //     .unwrap()
    //         //     .map(|a| a.unwrap())
    //         //     .collect::<Vec<Employee>>();
    //     }

    //     return all_employee;
    // }

    // pub fn get(id: i32) -> Option<Self> {
    //     let connection = database::connectionDatabase();

    //     if let Ok(conn) = connection {
    //         // let mut statement_get_all_employee = conn
    //         //     .query_map("select id, name from employee where id = :id")
    //         //     .expect("");

    //         // let mut row = statement_get_all_employee
    //         //     .query_named(named_params! {":id": id})
    //         //     .expect("");

    //         // if let Some(data) = row.next().expect("") {
    //         //     return Some(Employee {
    //         //         people_id: data.get(0).expect(""),
    //         //         name: data.get(1).expect(""),
    //         //     });
    //         // }
    //     }

    //     return None;
    // }
}
