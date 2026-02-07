use crate::db::traits::FromSql;

#[derive(Debug)]
pub struct SchemaWortGender {
    pub gender: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaWortGender {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, crate::helpers::error_handler::DbError> {
        Ok(Self {
            gender: r.get(0)?,
            created_at: r.get(1)?,
            deleted_at: r.get(2)?,
        })
    }
}

#[cfg(test)]
mod tests_schema_wort_gender {
    use super::*;
    use rusqlite::{Connection, params};

    #[test]
    fn from_sql_ok_with_deleted_at_null() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            r#"
            CREATE TABLE test (
                gender TEXT NOT NULL,
                created_at TEXT NOT NULL,
                deleted_at TEXT
            )
            "#,
            [],
        )
        .unwrap();

        conn.execute(
            r#"
            INSERT INTO test (gender, created_at, deleted_at)
            VALUES (?1, ?2, NULL)
            "#,
            params!["Maskuline", "2025-01-01 10:00:00"],
        )
        .unwrap();

        let mut stmt = conn
            .prepare("SELECT gender, created_at, deleted_at FROM test")
            .unwrap();
        let schema = stmt
            .query_row([], |row| SchemaWortGender::from_sql(row))
            .unwrap();

        assert_eq!(schema.gender, "Maskuline");
        assert_eq!(schema.created_at, "2025-01-01 10:00:00");
        assert_eq!(schema.deleted_at, None);
    }

    #[test]
    fn from_sql_ok_with_deleted_at_some() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            r#"
            CREATE TABLE test (
                gender TEXT NOT NULL,
                created_at TEXT NOT NULL,
                deleted_at TEXT
            )
            "#,
            [],
        )
        .unwrap();

        conn.execute(
            r#"
            INSERT INTO test (gender, created_at, deleted_at)
            VALUES (?1, ?2, ?3)
            "#,
            params!["Femenin", "2025-01-01 10:00:00", "2025-02-01 12:00:00"],
        )
        .unwrap();

        let mut stmt = conn
            .prepare("SELECT gender, created_at, deleted_at FROM test")
            .unwrap();
        let schema = stmt
            .query_row([], |row| SchemaWortGender::from_sql(row))
            .unwrap();

        assert_eq!(schema.gender, "Femenin");
        assert_eq!(schema.created_at, "2025-01-01 10:00:00");
        assert_eq!(schema.deleted_at, Some("2025-02-01 12:00:00".to_string()));
    }

    #[test]
    fn from_sql_fails_if_column_type_is_invalid() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            r#"
            CREATE TABLE test (
                gender INTEGER,
                created_at TEXT,
                deleted_at TEXT
            )
            "#,
            [],
        )
        .unwrap();

        conn.execute(
            r#"
            INSERT INTO test (gender, created_at, deleted_at)
            VALUES (123, '2025-01-01', NULL)
            "#,
            [],
        )
        .unwrap();

        let mut stmt = conn
            .prepare("SELECT gender, created_at, deleted_at FROM test")
            .unwrap();
        let err = stmt
            .query_row([], |row| SchemaWortGender::from_sql(row))
            .unwrap_err();

        // No comparamos string exacta porque rusqlite cambia mensajes según versión
        assert!(err.message.to_lowercase().contains("type"));
        assert!(err.source.is_some());
    }
}
