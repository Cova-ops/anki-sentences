use crate::db::traits::FromSql;

#[derive(Debug)]
pub struct SchemaGramType {
    pub code: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaGramType {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            code: r.get(0)?,
            created_at: r.get(1)?,
            deleted_at: r.get(2)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{db::queries::DbQuery, helpers::error_handler::DbError};

    use super::*;
    use rusqlite::{Connection, params};

    fn setup_db() -> Result<Connection, DbError> {
        let mut conn = Connection::open_in_memory()?;
        let tx = conn.transaction()?;

        DbQuery::execute(
            &tx,
            "
            CREATE TABLE gram_types (
                code TEXT NOT NULL,
                created_at TEXT NOT NULL,
                deleted_at TEXT NULL
            );
            ",
            [],
        )?;
        tx.commit()?;

        Ok(conn)
    }

    #[test]
    fn from_sql_reads_row_with_deleted_at_some() -> Result<(), DbError> {
        let mut conn = setup_db()?;
        let tx = conn.transaction()?;

        DbQuery::execute(
            &tx,
            "INSERT INTO gram_types (code, created_at, deleted_at) VALUES (?1, ?2, ?3)",
            params!["verb_main", "2026-02-05T00:00:00Z", "2026-02-06T00:00:00Z"],
        )?;

        let schema: SchemaGramType = DbQuery::query_one(
            &tx,
            "SELECT code, created_at, deleted_at FROM gram_types LIMIT 1",
            [],
            |row| SchemaGramType::from_sql(row),
        )?;
        tx.commit()?;

        assert_eq!(schema.code, "verb_main");
        assert_eq!(schema.created_at, "2026-02-05T00:00:00Z");
        assert_eq!(schema.deleted_at.as_deref(), Some("2026-02-06T00:00:00Z"));

        Ok(())
    }

    #[test]
    fn from_sql_reads_row_with_deleted_at_null_as_none() -> Result<(), DbError> {
        let mut conn = setup_db()?;
        let tx = conn.transaction()?;

        DbQuery::execute(
            &tx,
            "INSERT INTO gram_types (code, created_at, deleted_at) VALUES (?1, ?2, NULL)",
            params!["verb_main", "2026-02-05T00:00:00Z"],
        )?;

        let schema: SchemaGramType = DbQuery::query_one(
            &tx,
            "SELECT code, created_at, deleted_at FROM gram_types LIMIT 1",
            [],
            SchemaGramType::from_sql,
        )?;

        tx.commit()?;

        assert_eq!(schema.code, "verb_main");
        assert_eq!(schema.created_at, "2026-02-05T00:00:00Z");
        assert_eq!(schema.deleted_at, None);

        Ok(())
    }

    #[test]
    fn from_sql_errors_if_query_doesnt_return_all_columns() -> Result<(), DbError> {
        let mut conn = setup_db()?;
        let tx = conn.transaction()?;

        DbQuery::execute(
            &tx,
            "INSERT INTO gram_types (code, created_at, deleted_at) VALUES (?1, ?2, NULL)",
            params!["verb_main", "2026-02-05T00:00:00Z"],
        )?;

        let res: Result<SchemaGramType, DbError> = DbQuery::query_one(
            &tx,
            "SELECT code, created_at FROM gram_types LIMIT 1",
            [],
            SchemaGramType::from_sql,
        );

        tx.commit()?;

        assert!(res.is_err());

        Ok(())
    }
}
