use std::path::Path;

use rusqlite::Connection;

use crate::helpers::error_handler::DbError;

pub fn get_conn(path: &Path) -> Result<Connection, DbError> {
    let conn = Connection::open(path)?;

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
