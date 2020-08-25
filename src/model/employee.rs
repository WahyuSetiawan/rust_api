use rusqlite::{named_params, params};

use crate::database;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub people_id: i32,
    pub name: String,
}

impl Employee {
    // pub fn update(&self) -> Result<usize, String> {
    //     let connection_database = database::connection();

    //     if let Ok(connection) = connection_database {
    //         let status_update_query = connection.execute(
    //             "update employee set name = ?1 where id = ?0",
    //             params![self.people_id, self.name],
    //         );

    //         if let Ok(status) = status_update_query {
    //             return Ok(status);
    //         } else {
    //             return Err(format!(
    //                 "can't update employee, because error: {:?}",
    //                 status_update_query
    //             ));
    //         }
    //     } else {
    //         return Err(format!(
    //             "can't connect into database : {:?}",
    //             connection_database
    //         ));
    //     }
    // }

    // pub fn insert(&self) -> Result<usize, String> {
    //     let connection = database::connection();
    //     if let Ok(conn) = connection {
    //         let status_insert = conn.execute(
    //             "insert into employee (id, name) values (?1, ?2)",
    //             params![self.people_id, self.name],
    //         );

    //         if let Ok(status) = status_insert {
    //             return Ok(status);
    //         } else {
    //             println!("{:?}", status_insert);

    //             return Err("cannot to insert data".to_string());
    //         }
    //     } else {
    //         return Err(format!(
    //             "cant connect into database sqlite {:?}",
    //             connection
    //         ));
    //     }
    // }

    // pub fn getAll() -> Vec<Employee> {
    //     let mut all_employee: Vec<Employee> = Vec::new();
    //     let connection = database::connection();

    //     if let Ok(conn) = connection {
    //         let mut statement_get_all_employee =
    //             conn.prepare("select id, name from employee").unwrap();


    //         all_employee = statement_get_all_employee
    //             .query_map(params![], |row| {
    //                 Ok(Employee {
    //                     people_id: row.get(0).expect("not column for id 1"),
    //                     name: row.get(1).expect("not column for id 2"),
    //                 })
    //             })
    //             .unwrap()
    //             .map(|a| a.unwrap())
    //             .collect::<Vec<Employee>>();
    //     }

    //     return all_employee;
    // }

    // pub fn get(id: i32) -> Option<Self> {
    //     let connection = database::connection();

    //     if let Ok(conn) = connection {
    //         let mut statement_get_all_employee = conn
    //             .prepare("select id, name from employee where id = :id")
    //             .expect("");

    //         let mut row = statement_get_all_employee
    //             .query_named(named_params! {":id": id})
    //             .expect("");

    //         if let Some(data) = row.next().expect("") {
    //             return Some(Employee {
    //                 people_id: data.get(0).expect(""),
    //                 name: data.get(1).expect(""),
    //             });
    //         }
    //     }

    //     return None;
    // }
}
