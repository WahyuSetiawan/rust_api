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
        let connection_database = database::connectionDatabase();

        if let Ok(mut conn) = connection_database {
            let result = conn.as_mut().exec_drop(
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

        return Err("nothing todo with update employee".to_string());
    }

    pub fn insert(&self) -> Result<bool, String> {
        let connection = database::connectionDatabase();

        if let Ok(mut conn) = connection {
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

    pub fn getAll() -> Vec<Employee> {
        let mut all_employee: Vec<Employee> = Vec::new();
        let connection = database::connectionDatabase();

        if let Ok(mut conn) = connection {
            let data = conn.query_map("select id, name from employee", |(id, name)| Employee {
                people_id: id,
                name: name,
            });

            if let Ok(da) = data {
                all_employee = da;
            }
        }

        return all_employee;
    }

    pub fn get(id: i32) -> Option<Self> {
        let connection = database::connectionDatabase();

        if let Ok(mut conn) = connection {
            let data = conn.as_mut().query_map(
                format!("select id, name from employee where id = {}", id),
                |(id, name)| Employee {
                    people_id: id,
                    name: name,
                },
            );

            if let Ok(data1) = data {
                if data1.len() > 0 {
                    Some(data1.first());
                }
            }
        }

        return None;
    }
}
