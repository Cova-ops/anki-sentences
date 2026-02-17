use crate::db::traits::FromSql;

#[derive(Debug)]
pub struct SchemaWortGender {
    pub gender: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaWortGender {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            gender: r.get(0)?,
            created_at: r.get(1)?,
            deleted_at: r.get(2)?,
        })
    }
}

#[cfg(test)]
mod tests_schema_wort_gender {
    use rusqlite::Connection;

    use crate::{
        db::{schemas::wort_gender::SchemaWortGender, traits::FromSql},
        helpers::error_handler::DbError,
    };

    fn setup_db(sql_create: &'static str) -> Result<Connection, DbError> {
        let conn = Connection::open_in_memory()?;

        conn.execute(sql_create, [])?;

        Ok(conn)
    }

    #[test]
    fn from_sql_ok_with_deleted_at_null() -> Result<(), DbError> {
        let conn = setup_db(
            r#"
            CREATE TABLE test (
                gender TEXT NOT NULL,
                created_at TEXT NOT NULL,
                deleted_at TEXT
            );
            "#,
        )?;

        conn.execute(
            r#"
            INSERT INTO test (gender, created_at, deleted_at)
            VALUES (?1, ?2, NULL);
            "#,
            ("Maskuline", "2025-01-01 10:00:00"),
        )?;

        let schema: SchemaWortGender = conn.query_one(
            "SELECT gender, created_at, deleted_at FROM test",
            [],
            SchemaWortGender::from_sql,
        )?;

        assert_eq!(schema.gender, "Maskuline");
        assert_eq!(schema.created_at, "2025-01-01 10:00:00");
        assert_eq!(schema.deleted_at, None);

        Ok(())
    }

    #[test]
    fn from_sql_ok_with_deleted_at_some() -> Result<(), DbError> {
        let conn = setup_db(
            r#"
            CREATE TABLE test (
                gender TEXT NOT NULL,
                created_at TEXT NOT NULL,
                deleted_at TEXT
            );
            "#,
        )?;

        conn.execute(
            r#"
            INSERT INTO test (gender, created_at, deleted_at)
            VALUES (?1, ?2, ?3);
            "#,
            ("Femenin", "2025-01-01 10:00:00", "2025-02-01 12:00:00"),
        )?;

        let schema: SchemaWortGender = conn.query_one(
            "SELECT gender, created_at, deleted_at FROM test",
            [],
            SchemaWortGender::from_sql,
        )?;

        assert_eq!(schema.gender, "Femenin");
        assert_eq!(schema.created_at, "2025-01-01 10:00:00");
        assert_eq!(schema.deleted_at.as_deref(), Some("2025-02-01 12:00:00"));

        Ok(())
    }

    #[test]
    fn from_sql_fails_if_column_type_is_invalid() -> Result<(), DbError> {
        let conn = setup_db(
            r#"
            CREATE TABLE test (
                gender INTEGER,
                created_at TEXT,
                deleted_at TEXT
            );
            "#,
        )?;

        conn.execute(
            r#"
            INSERT INTO test (gender, created_at, deleted_at)
            VALUES (123, '2025-01-01', NULL);
            "#,
            [],
        )?;

        let err = conn
            .query_one(
                "SELECT gender, created_at, deleted_at FROM test",
                [],
                SchemaWortGender::from_sql,
            )
            .unwrap_err();

        match err {
            rusqlite::Error::InvalidColumnType(_, _, _) => {}
            _ => panic!("unexpected error"),
        }

        Ok(())
    }
}
