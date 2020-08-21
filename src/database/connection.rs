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
