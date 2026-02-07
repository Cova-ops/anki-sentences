use crate::db::traits::FromSql;

#[derive(Debug)]
pub struct SchemaGramType {
    pub code: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaGramType {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, crate::helpers::error_handler::DbError> {
        Ok(Self {
            code: r.get(0)?,
            created_at: r.get(1)?,
            deleted_at: r.get(2)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{Connection, params};

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            r#"
            CREATE TABLE gram_types (
                code TEXT NOT NULL,
                created_at TEXT NOT NULL,
                deleted_at TEXT NULL
            );
            "#,
            [],
        )
        .unwrap();

        conn
    }

    #[test]
    fn from_sql_reads_row_with_deleted_at_some() {
        let conn = setup_db();

        conn.execute(
            "INSERT INTO gram_types (code, created_at, deleted_at) VALUES (?1, ?2, ?3)",
            params!["verb_main", "2026-02-05T00:00:00Z", "2026-02-06T00:00:00Z"],
        )
        .unwrap();

        let mut stmt = conn
            .prepare("SELECT code, created_at, deleted_at FROM gram_types LIMIT 1")
            .unwrap();

        let schema: SchemaGramType = stmt
            .query_row([], |row| SchemaGramType::from_sql(row))
            .unwrap();

        assert_eq!(schema.code, "verb_main");
        assert_eq!(schema.created_at, "2026-02-05T00:00:00Z");
        assert_eq!(schema.deleted_at.as_deref(), Some("2026-02-06T00:00:00Z"));
    }

    #[test]
    fn from_sql_reads_row_with_deleted_at_null_as_none() {
        let conn = setup_db();

        conn.execute(
            "INSERT INTO gram_types (code, created_at, deleted_at) VALUES (?1, ?2, NULL)",
            params!["verb_main", "2026-02-05T00:00:00Z"],
        )
        .unwrap();

        let mut stmt = conn
            .prepare("SELECT code, created_at, deleted_at FROM gram_types LIMIT 1")
            .unwrap();

        let schema: SchemaGramType = stmt
            .query_row([], |row| SchemaGramType::from_sql(row))
            .unwrap();

        assert_eq!(schema.code, "verb_main");
        assert_eq!(schema.created_at, "2026-02-05T00:00:00Z");
        assert_eq!(schema.deleted_at, None);
    }

    #[test]
    fn from_sql_errors_if_query_doesnt_return_all_columns() {
        let conn = setup_db();

        conn.execute(
            "INSERT INTO gram_types (code, created_at, deleted_at) VALUES (?1, ?2, NULL)",
            params!["verb_main", "2026-02-05T00:00:00Z"],
        )
        .unwrap();

        // OJO: aquí pedimos solo 2 columnas, pero tu FromSql hace r.get(2)?
        let mut stmt = conn
            .prepare("SELECT code, created_at FROM gram_types LIMIT 1")
            .unwrap();

        let res = stmt.query_row::<SchemaGramType>([], |row| SchemaGramType::from_sql(row));

        assert!(res.is_err());
    }
}
