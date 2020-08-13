// use mysql::*;
// use mysql::prolude::*;

// #derive(Debug, PartialEq, Eq)
// struct People{
//     people_id: i32,
//     name: Option<String>,
// }

// let url = "msyql://root:@localhost:3307/rust_test";

// let pool =  Pool::new(url)?;

// let mut conn = pool.get_conn()?;

// conn.query_drop(
//     r"CREATE TEMPORARY TABLE payment (
//         customer_id int not null,
//         amount int not null,
//         account_name text
//     )")?;

// let payments = vec![
//     People { people_id: 1, name: Some("wahyu setiawan") },
// ];

// conn.exec_batch(
//     r"Insert into people (people_id, name) values (:pople_id, :name)",
//     payments.iter().map(|p| params! {
//         "people_id" => p.people_id,
//         "name" => &p.name
//     })
// )?;

// // Let's select payments from database. Type inference should do the trick here.
// let selected_payments = conn
//     .query_map(
//         "select people_id, name from people",
//         |(people_id, name)| {
//             People {people_id, name}
//         },
//     )?;

// // Let's make sure, that `payments` equals to `selected_payments`.
// // Mysql gives no guaranties on order of returned rows
// // without `ORDER BY`, so assume we are lucky.
// assert_eq!(payments, selected_payments);
// println!("Yay!");

// use rusqlite::{Connection, Result};
// use rusqlite::NO_PARAMS;

// let conn = Connection::open("");

use rusqlite::Connection;

use std::env;

pub fn connection() -> Result<Connection, String> {
    let path = env::current_dir();

    if let Ok(path_database) = path {
        let path_database_employee: String =
            format!("{}{}", path_database.display(), "\\employe.db");

        let connection = Connection::open(path_database_employee);

        if let Ok(connection_database) = connection {
            return Ok(connection_database);
        } else {
            return Err("not gettering connection on database".to_string());
        }
    } else {
        return Err("error getting path".to_string());
    }
}
