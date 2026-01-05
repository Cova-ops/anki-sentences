use std::path::Path;

use color_eyre::eyre::Result;
use rusqlite::Connection;

pub fn get_conn(path: &Path) -> Result<Connection> {
    let conn = Connection::open(path)?;

    // Buenas prácticas SQLite
    conn.execute_batch(
        r#"
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA foreign_keys = ON;
        PRAGMA busy_timeout = 5000;
        "#,
    )?;

    Ok(conn)
}

#[cfg(test)]
pub fn setup_test_db() -> Result<Connection> {
    use crate::db::schemas::init_schemas;

    let mut conn = Connection::open_in_memory()?;
    init_schemas(&mut conn)?;

    Ok(conn)
}
