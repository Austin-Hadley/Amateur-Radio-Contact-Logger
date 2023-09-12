use rusqlite::{Connection, Result};

pub fn create_index_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS index (
            id INTEGER PRIMARY KEY,
            callsign TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;
    return conn;
}

pub fn create_name_table(conn: &Connection) -> Result<()> {
    // create the table and create a field that takes the primary key from the index table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS name (
            index_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (index_id) REFERENCES index (id)
        )",
        [],
    )?;
    return conn;
}

pub fn create_freq_table(conn: &Connection) -> Result<()> {
    // create the table and create a field that takes the primary key from the index table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS freq (
            index_id INTEGER NOT NULL,
            freq TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (index_id) REFERENCES index (id)
        )",
        [],
    )?;
    return conn;
}

pub fn create_mode_table(conn: &Connection) -> Result<()> {
    // create the table and create a field that takes the primary key from the index table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS mode (
            index_id INTEGER NOT NULL,
            mode TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (index_id) REFERENCES index (id)
        )",
        [],
    )?;
    return conn;
}

pub fn create_rst_table(conn: &Connection) -> Result<()> {
    // create columns for a sent and received rst report
    conn.execute(
        "CREATE TABLE IF NOT EXISTS rst (
            index_id INTEGER NOT NULL,
            sent_rst TEXT NOT NULL,
            received_rst TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (index_id) REFERENCES index (id)
        )",
        [],
    )?;
}

pub fn create_qth_table(conn: &Connection) -> Result<()> {
    // create the table and create a field that takes the primary key from the index table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS qth (
            index_id INTEGER NOT NULL,
            qth TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (index_id) REFERENCES index (id)
        )",
        [],
    )?;
    return conn;
}

pub fn create_comments_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS comments (
            index_id INTEGER NOT NULL,
            comments TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (index_id) REFERENCES index (id)
        )",
        [],
    )
}

pub fn create_tx_pwr_table(conn: &Connection) -> Result<()> {
    // create the table and create a field that takes the primary key from the index table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tx_pwr (
            index_id INTEGER NOT NULL,
            tx_pwr TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (index_id) REFERENCES index (id)
        )",
        [],
    )?;
    return conn;
}

pub fn create_band_table(conn: &Connection) -> Result<()> {
    // create the table and create a field that takes the primary key from the index table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS band (
            index_id INTEGER NOT NULL,
            band TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (index_id) REFERENCES index (id)
        )",
        [],
    )?;
    return conn;
}

pub fn create_all_tables(conn: &Connection) -> Result<()> {
    create_index_table(&conn);
    create_name_table(&conn);
    create_freq_table(&conn);
    create_mode_table(&conn);
    create_rst_table(&conn);
    create_qth_table(&conn);
    create_comments_table(&conn);
    create_tx_pwr_table(&conn);
    create_band_table(&conn);
    return conn;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_index_table() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_all_tables(&conn);
        Ok(())
    }
}