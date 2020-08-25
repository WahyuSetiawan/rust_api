use rusqlite::Connection;

use std::env;

use crate::env as envFileUse;
use mysql::*;

mod embedded {
    use refinery::embed_migrations;

    embed_migrations!("migration");
}

// pub fn connection() -> Result<Connection, String> {
//     let path = env::current_dir();

//     if let Ok(path_database) = path {
//         let path_database_employee: String =
//             format!("{}{}", path_database.display(), "\\employe.db");

//         let connection = Connection::open(path_database_employee);

//         if let Ok(connection_database) = connection {
//             return Ok(connection_database);
//         } else {
//             return Err("not gettering connection on database".to_string());
//         }
//     } else {
//         return Err("error getting path".to_string());
//     }
// }

pub fn connectionDatabase() -> PooledConn {
    let mut envfile = unsafe { envFileUse::envComponentSingleton.getData() };
    let data = &envfile.store;

    let url = format! {"mysql://{}@{}:{}/{}", data["username"], data["database_server"], data["port"], data["database"]};

    println!("{}", url);

    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    embedded::migrations::runner().run(&mut conn).unwrap();

    return conn;
}
