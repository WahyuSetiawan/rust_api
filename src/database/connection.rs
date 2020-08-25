use crate::env as envFileUse;

use mysql::*;

mod embedded {
    use refinery::embed_migrations;

    embed_migrations!("migration");
}

pub fn connectionDatabase() -> Result<PooledConn> {
    let mut envfile = unsafe { envFileUse::envComponentSingleton.getData() };
    let data = &envfile.store;

    let url = format! {"mysql://{}@{}:{}/{}", data["username"], data["database_server"], data["port"], data["database"]};

    println!("{}", url);

    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    embedded::migrations::runner().run(&mut conn).unwrap();

    return Ok(conn);
}
