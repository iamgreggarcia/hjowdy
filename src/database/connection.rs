use rusqlite::{params, Connection, Result};

pub struct DbPoolOptions {
    max_connections: u32,
}

pub fn connect() -> Result<Connection> {
    let conn = Connection::open("my_database.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS chat_history (
            id TEXT PRIMARY KEY,
            messages TEXT
        )",
        params![],
    )?;
    Ok(conn)
}
