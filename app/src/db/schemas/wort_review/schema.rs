use crate::db::traits::FromSql;

#[derive(Debug)]
pub struct SchemaWortReview {
    pub id: i32,
    pub wort_id: i32,
    pub direction: String,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: String,
    pub next_review: String,

    // Generic
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaWortReview {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: r.get(0)?,
            wort_id: r.get(1)?,
            direction: r.get(2)?,
            interval: r.get(3)?,
            ease_factor: r.get(4)?,
            repetitions: r.get(5)?,
            last_review: r.get(6)?,
            next_review: r.get(7)?,
            created_at: r.get(8)?,
            deleted_at: r.get(9)?,
        })
    }
}

#[cfg(test)]
mod tests_schema_wort_review_from_sql {
    use super::*;
    use crate::helpers::error_handler::DbError;
    use rusqlite::Connection;

    #[test]
    fn from_sql_ok_deleted_at_null() -> Result<(), DbError> {
        let conn = Connection::open_in_memory()?;

        let sql = r#"
            SELECT
                1,              -- id (i32)
                188,            -- wort_id (i32)
                'es_to_de',     -- direction (String)
                10,             -- interval (u32)
                2.5,            -- ease_factor (f32)
                3,              -- repetitions (u32)
                '2025-12-01 00:00:00', -- last_review (String)
                '2025-12-02 00:00:00', -- next_review (String)
                '2025-12-01 00:00:00', -- created_at (String)
                NULL            -- deleted_at (Option<String>)
        "#;

        let schema: SchemaWortReview = conn.query_one(sql, [], SchemaWortReview::from_sql)?;

        assert_eq!(schema.id, 1);
        assert_eq!(schema.wort_id, 188);
        assert_eq!(schema.direction, "es_to_de");
        assert_eq!(schema.interval, 10);
        assert!((schema.ease_factor - 2.5).abs() < f32::EPSILON);
        assert_eq!(schema.repetitions, 3);
        assert_eq!(schema.last_review, "2025-12-01 00:00:00");
        assert_eq!(schema.next_review, "2025-12-02 00:00:00");
        assert_eq!(schema.created_at, "2025-12-01 00:00:00");
        assert_eq!(schema.deleted_at, None);

        Ok(())
    }

    #[test]
    fn from_sql_ok_deleted_at_some() -> Result<(), DbError> {
        let conn = Connection::open_in_memory()?;

        let sql = r#"
            SELECT
                2,
                189,
                'de_to_es',
                1,
                2.3,
                0,
                '2025-12-01 10:00:00',
                '2025-12-01 12:00:00',
                '2025-12-01 10:00:00',
                '2025-12-31 23:59:59'
        "#;

        let schema: SchemaWortReview = conn.query_one(sql, [], SchemaWortReview::from_sql)?;

        assert_eq!(schema.id, 2);
        assert_eq!(schema.wort_id, 189);
        assert_eq!(schema.direction, "de_to_es");
        assert_eq!(schema.interval, 1);
        assert!((schema.ease_factor - 2.3).abs() < f32::EPSILON);
        assert_eq!(schema.repetitions, 0);
        assert_eq!(schema.last_review, "2025-12-01 10:00:00");
        assert_eq!(schema.next_review, "2025-12-01 12:00:00");
        assert_eq!(schema.created_at, "2025-12-01 10:00:00");
        assert_eq!(schema.deleted_at.as_deref(), Some("2025-12-31 23:59:59"));

        Ok(())
    }

    #[test]
    fn from_sql_err_type_mismatch() -> Result<(), DbError> {
        let conn = Connection::open_in_memory()?;

        // interval debería ser número, pero metemos texto para forzar error
        let sql = r#"
            SELECT
                1,
                188,
                'es_to_de',
                'NOT_A_NUMBER',
                2.5,
                3,
                '2025-12-01 00:00:00',
                '2025-12-02 00:00:00',
                '2025-12-01 00:00:00',
                NULL
        "#;

        let res: Result<SchemaWortReview, DbError> = conn
            .query_one(sql, [], SchemaWortReview::from_sql)
            .map_err(Into::into);

        assert!(res.is_err(), "should fail due to type mismatch");

        Ok(())
    }
}
