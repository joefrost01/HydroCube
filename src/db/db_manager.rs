use duckdb::{Connection, Error};

pub struct DbManager {
    pub conn: Connection,
}

impl DbManager {
    pub fn new(db_path: &str) -> Result<Self, Error> {
        let conn = Connection::open(db_path)?;
        Ok(DbManager { conn })
    }
}
