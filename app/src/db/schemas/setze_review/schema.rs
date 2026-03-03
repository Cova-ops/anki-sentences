use crate::db::traits::FromSql;

#[derive(Debug)]
pub struct SchemaSetzeReview {
    pub satz_id: i32,
    pub direction: String,
    pub interval: u32,
    pub ease_factor: f64,
    pub repetitions: u32,
    pub last_review: String,
    pub next_review: String,

    // Generic
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaSetzeReview {
    /// Orden:
    /// - satz_id
    /// - direction
    /// - interval
    /// - ease_factor
    /// - repetitions
    /// - last_review
    /// - next_review
    /// - created_at
    /// - deleted_at
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            satz_id: r.get(0)?,
            direction: r.get(1)?,
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
mod tests {
    use super::*;

    use crate::helpers::error_handler::DbError;
    use rusqlite::Connection;

    mod from_sql {
        use super::*;

        #[test]
        fn ok_with_null_deleted_at() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            let mut stmt = conn.prepare(
                "SELECT 10, 'es_to_de', 3, 2.5, 7, '2025-12-04 20:00:00', '2025-12-10 20:00:00', '2025-12-04 20:00:00', NULL;",
            )?;

            let out: SchemaSetzeReview = stmt.query_one([], SchemaSetzeReview::from_sql)?;

            assert_eq!(out.satz_id, 10);
            assert_eq!(out.direction, "es_to_de");
            assert_eq!(out.interval, 3);
            assert!((out.ease_factor - 2.5).abs() < f64::EPSILON);
            assert_eq!(out.repetitions, 7);
            assert_eq!(out.last_review, "2025-12-04 20:00:00");
            assert_eq!(out.next_review, "2025-12-10 20:00:00");
            assert_eq!(out.created_at, "2025-12-04 20:00:00");
            assert_eq!(out.deleted_at, None);

            Ok(())
        }

        #[test]
        fn ok_with_some_deleted_at() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            let mut stmt = conn.prepare(
                "SELECT 11, 'es_to_de', 1, 2.3, 1, '2025-12-04 20:00:00', '2025-12-05 20:00:00', '2025-12-04 20:00:00', '2025-12-31 00:00:00';",
            )?;

            let out: SchemaSetzeReview = stmt.query_one([], SchemaSetzeReview::from_sql)?;

            assert_eq!(out.satz_id, 11);
            assert_eq!(out.direction, "es_to_de");
            assert_eq!(out.interval, 1);
            assert!((out.ease_factor - 2.3).abs() < f64::EPSILON);
            assert_eq!(out.repetitions, 1);
            assert_eq!(out.last_review, "2025-12-04 20:00:00");
            assert_eq!(out.next_review, "2025-12-05 20:00:00");
            assert_eq!(out.created_at, "2025-12-04 20:00:00");
            assert_eq!(out.deleted_at.as_deref(), Some("2025-12-31 00:00:00"));

            Ok(())
        }

        #[test]
        fn err_type_mismatch() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            // interval should be INTEGER (u32), but we provide TEXT
            let mut stmt = conn.prepare(
                "SELECT 10, 'es_to_de', 'oops', 2.5, 7, '2025-12-04 20:00:00', '2025-12-10 20:00:00', '2025-12-04 20:00:00', NULL;"
            )?;

            let out: Result<SchemaSetzeReview, _> = stmt.query_one([], SchemaSetzeReview::from_sql);

            assert!(out.is_err());
            let err = out.unwrap_err();

            match err {
                rusqlite::Error::InvalidColumnType(_, _, _) => {}
                other => panic!("Unexpected error: {other:?}"),
            }

            Ok(())
        }

        #[test]
        fn err_missing_column() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            // deleted_at column missing (index 8)
            let mut stmt = conn.prepare(
                "SELECT 10, 'es_to_de', 3, 2.5, 7, '2025-12-04 20:00:00', '2025-12-10 20:00:00', '2025-12-04 20:00:00';",
            )?;

            let out: Result<SchemaSetzeReview, _> = stmt.query_one([], SchemaSetzeReview::from_sql);

            assert!(out.is_err());
            let err = out.unwrap_err();

            match err {
                rusqlite::Error::InvalidColumnIndex(_) => {}
                other => panic!("Unexpected error: {other:?}"),
            }

            Ok(())
        }
    }
}
