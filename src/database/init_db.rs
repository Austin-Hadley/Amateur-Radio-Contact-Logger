use rusqlite::{Connection, Result};

pub fn create_db(filename: &str) -> Result<()> {
    let conn = Connection::open(filename)?;
    return conn;
}