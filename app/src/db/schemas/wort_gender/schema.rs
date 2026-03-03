use crate::db::traits::FromSql;

#[derive(Debug)]
pub struct SchemaWortGender {
    pub gender: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaWortGender {
    /// Orden:
    /// - gender
    /// - created_at
    /// - deleted_at
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            gender: r.get(0)?,
            created_at: r.get(1)?,
            deleted_at: r.get(2)?,
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

            let mut stmt = conn.prepare("SELECT 'der', '2025-12-04 20:00:00', NULL;")?;
            let out: SchemaWortGender = stmt.query_one([], SchemaWortGender::from_sql)?;

            assert_eq!(out.gender, "der");
            assert_eq!(out.created_at, "2025-12-04 20:00:00");
            assert_eq!(out.deleted_at, None);

            Ok(())
        }

        #[test]
        fn ok_with_some_deleted_at() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            let mut stmt =
                conn.prepare("SELECT 'die', '2025-12-04 20:00:00', '2025-12-31 00:00:00';")?;
            let out: SchemaWortGender = stmt.query_one([], SchemaWortGender::from_sql)?;

            assert_eq!(out.gender, "die");
            assert_eq!(out.created_at, "2025-12-04 20:00:00");
            assert_eq!(out.deleted_at.as_deref(), Some("2025-12-31 00:00:00"));

            Ok(())
        }

        #[test]
        fn err_type_mismatch() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            // gender should be TEXT but we provide INTEGER
            let mut stmt = conn.prepare("SELECT 123, '2025-12-04 20:00:00', NULL;")?;
            let out: Result<SchemaWortGender, _> = stmt.query_one([], SchemaWortGender::from_sql);

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

            // deleted_at column missing
            let mut stmt = conn.prepare("SELECT 'das', '2025-12-04 20:00:00';")?;
            let out: Result<SchemaWortGender, _> = stmt.query_one([], SchemaWortGender::from_sql);

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
