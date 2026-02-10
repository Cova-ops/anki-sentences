use crate::db::traits::FromSql;

#[derive(Debug)]
pub struct SchemaSetzeReview {
    pub id: i32,
    pub satz_id: i32,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: String,
    pub next_review: String,

    // Generic
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaSetzeReview {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: r.get(0)?,
            satz_id: r.get(1)?,
            interval: r.get(2)?,
            ease_factor: r.get(3)?,
            repetitions: r.get(4)?,
            last_review: r.get(5)?,
            next_review: r.get(6)?,
            created_at: r.get(7)?,
            deleted_at: r.get(8)?,
        })
    }
}

#[cfg(test)]
mod tests_schema_setze_review {
    use crate::test_utils::prelude::*;
    use rusqlite::Connection;

    fn setup_db() -> Result<Connection, DbError> {
        let conn = Connection::open_in_memory()?;

        conn.execute(
            r#"
            CREATE TABLE setze_review (
                id INTEGER,
                satz_id INTEGER,
                interval INTEGER,
                ease_factor REAL,
                repetitions INTEGER,
                last_review TEXT,
                next_review TEXT,
                created_at TEXT,
                deleted_at TEXT
            );
            "#,
            [],
        )?;

        conn.execute(
            r#"
            INSERT INTO setze_review VALUES (
                1,
                42,
                3,
                2.5,
                7,
                '2025-01-01 10:00:00',
                '2025-01-04 10:00:00',
                '2025-01-01 09:00:00',
                NULL
            );
            "#,
            [],
        )?;

        Ok(conn)
    }

    #[test]
    fn from_sql_maps_all_fields_correctly() -> Result<(), DbError> {
        let conn = setup_db()?;

        let sql = r#"
            SELECT
                id,
                satz_id,
                interval,
                ease_factor,
                repetitions,
                last_review,
                next_review,
                created_at,
                deleted_at
            FROM setze_review
        "#;

        let schema: SchemaSetzeReview = conn.query_one(sql, [], SchemaSetzeReview::from_sql)?;

        assert_eq!(schema.id, 1);
        assert_eq!(schema.satz_id, 42);
        assert_eq!(schema.interval, 3);
        assert_eq!(schema.ease_factor, 2.5);
        assert_eq!(schema.repetitions, 7);
        assert_eq!(schema.last_review, "2025-01-01 10:00:00");
        assert_eq!(schema.next_review, "2025-01-04 10:00:00");
        assert_eq!(schema.created_at, "2025-01-01 09:00:00");
        assert_eq!(schema.deleted_at, None);

        Ok(())
    }

    #[test]
    fn from_sql_handles_deleted_at_present() -> Result<(), DbError> {
        let conn = Connection::open_in_memory()?;

        conn.execute(
            r#"
            CREATE TABLE setze_review (
                id INTEGER,
                satz_id INTEGER,
                interval INTEGER,
                ease_factor REAL,
                repetitions INTEGER,
                last_review TEXT,
                next_review TEXT,
                created_at TEXT,
                deleted_at TEXT
            );
            "#,
            [],
        )?;

        conn.execute(
            r#"
            INSERT INTO setze_review VALUES (
                2,
                99,
                1,
                1.8,
                0,
                '2025-02-01 12:00:00',
                '2025-02-02 12:00:00',
                '2025-02-01 11:00:00',
                '2025-02-10 00:00:00'
            );
            "#,
            [],
        )?;

        let schema: SchemaSetzeReview = conn.query_one(
            "SELECT * FROM setze_review",
            [],
            SchemaSetzeReview::from_sql,
        )?;

        assert_eq!(schema.id, 2);
        assert_eq!(schema.deleted_at.as_deref(), Some("2025-02-10 00:00:00"));

        Ok(())
    }
}
