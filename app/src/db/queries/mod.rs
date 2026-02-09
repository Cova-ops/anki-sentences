use rusqlite::OptionalExtension;

use crate::helpers::error_handler::DbError;

pub struct DbQuery;

impl DbQuery {
    pub fn query_one<T, P, F>(
        tx: &rusqlite::Transaction,
        sql: &'static str,
        params: P,
        f: F,
    ) -> Result<T, DbError>
    where
        P: rusqlite::Params,
        F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
    {
        let mut stmt = tx.prepare(sql).map_err(|e| DbError {
            sql: Some(sql),
            message: e.to_string(),
            source: Some(e),
        })?;

        stmt.query_row(params, f).map_err(|e| DbError {
            sql: Some(sql),
            message: e.to_string(),
            source: Some(e),
        })
    }

    pub fn query_all<T, P, F>(
        tx: &rusqlite::Transaction,
        sql: &'static str,
        params: P,
        f: F,
    ) -> Result<Vec<T>, DbError>
    where
        P: rusqlite::Params,
        F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
    {
        let mut stmt = tx.prepare(sql).map_err(|e| DbError {
            sql: Some(sql),
            message: e.to_string(),
            source: Some(e),
        })?;

        stmt.query_map(params, f).map_err(|e| DbError {
            sql: Some(sql),
            message: e.to_string(),
            source: Some(e),
        })
    }

    pub fn query_optional<T, P, F>(
        tx: &rusqlite::Transaction,
        sql: &'static str,
        params: P,
        f: F,
    ) -> Result<Option<T>, DbError>
    where
        P: rusqlite::Params,
        F: FnOnce(&rusqlite::Row<'_>) -> rusqlite::Result<T>,
    {
        let mut stmt = tx.prepare(sql).map_err(|e| DbError {
            sql: Some(sql),
            message: e.to_string(),
            source: Some(e),
        })?;

        stmt.query_row(params, f).optional().map_err(|e| DbError {
            sql: Some(sql),
            message: e.to_string(),
            source: Some(e),
        })
    }

    pub fn execute<P, F>(
        tx: &rusqlite::Transaction,
        sql: &'static str,
        params: P,
    ) -> Result<usize, DbError>
    where
        P: rusqlite::Params,
    {
        let mut stmt = tx.prepare(sql).map_err(|e| DbError {
            sql: Some(sql),
            message: e.to_string(),
            source: Some(e),
        })?;

        stmt.execute(params).map_err(|e| DbError {
            sql: Some(sql),
            message: e.to_string(),
            source: Some(e),
        })
    }
}

#[cfg(test)]
mod db_query_tests {
    use super::*;
    use rusqlite::{Connection, params};

    fn setup_conn() -> Result<Connection, DbError> {
        let conn = Connection::open_in_memory()?;
        Ok(conn)
    }

    fn setup_schema(tx: &rusqlite::Transaction) -> Result<(), DbError> {
        DbQuery::execute(
            tx,
            "CREATE TABLE items (id INTEGER PRIMARY KEY, name TEXT NOT NULL);",
            [],
        )?;
        Ok(())
    }

    // -------------------------
    // execute()
    // -------------------------
    #[test]
    fn execute_inserts_and_returns_rows_affected() -> Result<(), DbError> {
        let mut conn = setup_conn()?;
        let tx = conn.transaction()?;
        setup_schema(&tx)?;

        let n = DbQuery::execute(
            &tx,
            "INSERT INTO items (id, name) VALUES (?1, ?2);",
            params![1, "abc"],
        )?;

        tx.commit()?;
        assert_eq!(n, 1);
        Ok(())
    }

    #[test]
    fn execute_returns_db_error_with_sql_on_prepare_failure() -> Result<(), DbError> {
        let mut conn = setup_conn()?;
        let tx = conn.transaction()?;

        static SQL: &str = "INSRT INTO items (id) VALUES (1)"; // SQL inválido

        let err = DbQuery::execute(&tx, SQL, []).unwrap_err();
        assert_eq!(err.sql, Some(SQL));
        assert!(err.source.is_some());

        tx.rollback()?;
        Ok(())
    }

    // -------------------------
    // query_one()
    // -------------------------
    #[test]
    fn query_one_reads_single_row() -> Result<(), DbError> {
        let mut conn = setup_conn()?;
        let tx = conn.transaction()?;
        setup_schema(&tx)?;

        DbQuery::execute(
            &tx,
            "INSERT INTO items (id, name) VALUES (?1, ?2);",
            params![1, "abc"],
        )?;

        let name: String = DbQuery::query_one(
            &tx,
            "SELECT name FROM items WHERE id = ?1;",
            params![1],
            |row| row.get(0),
        )?;

        tx.commit()?;
        assert_eq!(name, "abc");
        Ok(())
    }

    #[test]
    fn query_one_returns_error_when_no_rows() -> Result<(), DbError> {
        let mut conn = setup_conn()?;
        let tx = conn.transaction()?;
        setup_schema(&tx)?;

        let res: Result<String, DbError> = DbQuery::query_one(
            &tx,
            "SELECT name FROM items WHERE id = ?1;",
            params![999],
            |row| row.get(0),
        );

        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.sql, Some("SELECT name FROM items WHERE id = ?1;"));
        assert!(err.source.is_some());

        tx.rollback()?;
        Ok(())
    }

    #[test]
    fn query_one_returns_db_error_with_sql_on_prepare_failure() -> Result<(), DbError> {
        let mut conn = setup_conn()?;
        let tx = conn.transaction()?;

        static SQL: &str = "SELEC name FROM items"; // SQL inválido
        let err = DbQuery::query_one(&tx, SQL, [], |row| row.get::<_, String>(0)).unwrap_err();

        assert_eq!(err.sql, Some(SQL));
        assert!(err.source.is_some());

        tx.rollback()?;
        Ok(())
    }

    // -------------------------
    // query_all()
    // -------------------------
    #[test]
    fn query_all_reads_multiple_rows() -> Result<(), DbError> {
        let mut conn = setup_conn()?;
        let tx = conn.transaction()?;
        setup_schema(&tx)?;

        DbQuery::execute(
            &tx,
            "INSERT INTO items (id, name) VALUES (?1, ?2);",
            params![1, "a"],
        )?;
        DbQuery::execute(
            &tx,
            "INSERT INTO items (id, name) VALUES (?1, ?2);",
            params![2, "b"],
        )?;
        DbQuery::execute(
            &tx,
            "INSERT INTO items (id, name) VALUES (?1, ?2);",
            params![3, "c"],
        )?;

        let names: Vec<String> =
            DbQuery::query_all(&tx, "SELECT name FROM items ORDER BY id;", [], |row| {
                row.get(0)
            })?;

        tx.commit()?;
        assert_eq!(
            names,
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
        Ok(())
    }

    #[test]
    fn query_all_returns_db_error_with_sql_on_prepare_failure() -> Result<(), DbError> {
        let mut conn = setup_conn()?;
        let tx = conn.transaction()?;

        static SQL: &str = "SELEC name FROM items"; // inválido
        let res: Result<Vec<String>, DbError> = DbQuery::query_all(&tx, SQL, [], |row| row.get(0));

        let err = res.unwrap_err();
        assert_eq!(err.sql, Some(SQL));
        assert!(err.source.is_some());

        tx.rollback()?;
        Ok(())
    }

    // -------------------------
    // query_optional()
    // -------------------------
    #[test]
    fn query_optional_returns_some_when_row_exists() -> Result<(), DbError> {
        let mut conn = setup_conn()?;
        let tx = conn.transaction()?;
        setup_schema(&tx)?;

        DbQuery::execute(
            &tx,
            "INSERT INTO items (id, name) VALUES (?1, ?2);",
            params![1, "abc"],
        )?;

        let name: Option<String> = DbQuery::query_optional(
            &tx,
            "SELECT name FROM items WHERE id = ?1;",
            params![1],
            |row| row.get(0),
        )?;

        tx.commit()?;
        assert_eq!(name.as_deref(), Some("abc"));
        Ok(())
    }

    #[test]
    fn query_optional_returns_none_when_no_row() -> Result<(), DbError> {
        let mut conn = setup_conn()?;
        let tx = conn.transaction()?;
        setup_schema(&tx)?;

        let name: Option<String> = DbQuery::query_optional(
            &tx,
            "SELECT name FROM items WHERE id = ?1;",
            params![999],
            |row| row.get(0),
        )?;

        tx.commit()?;
        assert_eq!(name, None);
        Ok(())
    }

    #[test]
    fn query_optional_returns_db_error_with_sql_on_prepare_failure() -> Result<(), DbError> {
        let mut conn = setup_conn()?;
        let tx = conn.transaction()?;

        static SQL: &str = "SELEC name FROM items"; // inválido
        let res: Result<Option<String>, DbError> =
            DbQuery::query_optional(&tx, SQL, [], |row| row.get(0));

        let err = res.unwrap_err();
        assert_eq!(err.sql, Some(SQL));
        assert!(err.source.is_some());

        tx.rollback()?;
        Ok(())
    }
}
