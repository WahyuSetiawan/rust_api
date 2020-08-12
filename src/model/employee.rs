use rusqlite::{Connection, params};

use crate::database;
pub struct Employee{
    pub people_id : i32,
    pub name: String,
}

impl Employee {
    pub fn insert(&self) -> Result<usize, String>{
        let connection = database::connection();
        if let Ok(conn) = connection {
                let status_insert  = conn.execute("insert into employee (id, name) values (?1, ?2)", params![
                    self.people_id,
                    self.name
                ]);

                if let Ok(status) = status_insert{
                    return Ok(status);
                }else{
                    println!("{:?}", status_insert);

                    return Err("cannot to insert data".to_string());
                }
        }else{
            return Err(format!("cant connect into database sqlite {:?}", connection));
        }
    }
}