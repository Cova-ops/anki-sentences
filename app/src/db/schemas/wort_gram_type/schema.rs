use crate::db::traits::FromSql;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaWortGramType {
    pub id_worte: i32,
    pub id_gram_type: i32,

    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaWortGramType {
    /// Orden:
    /// - id_worte
    /// - id_gram_type
    /// - created_at
    /// - deleted_at
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id_worte: r.get(0)?,
            id_gram_type: r.get(1)?,

            created_at: r.get(2)?,
            deleted_at: r.get(3)?,
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

            let mut stmt = conn.prepare("SELECT 10, 3, '2025-12-04 20:00:00', NULL;")?;

            let out: SchemaWortGramType = stmt.query_one([], SchemaWortGramType::from_sql)?;

            assert_eq!(out.id_worte, 10);
            assert_eq!(out.id_gram_type, 3);
            assert_eq!(out.created_at, "2025-12-04 20:00:00");
            assert_eq!(out.deleted_at, None);

            Ok(())
        }

        #[test]
        fn ok_with_some_deleted_at() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            let mut stmt =
                conn.prepare("SELECT 11, 4, '2025-12-04 20:00:00', '2025-12-31 00:00:00';")?;

            let out: SchemaWortGramType = stmt.query_one([], SchemaWortGramType::from_sql)?;

            assert_eq!(out.id_worte, 11);
            assert_eq!(out.id_gram_type, 4);
            assert_eq!(out.created_at, "2025-12-04 20:00:00");
            assert_eq!(out.deleted_at.as_deref(), Some("2025-12-31 00:00:00"));

            Ok(())
        }

        #[test]
        fn err_type_mismatch() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            // id_worte should be INTEGER but we provide TEXT
            let mut stmt = conn.prepare("SELECT 'wrong', 3, '2025-12-04 20:00:00', NULL;")?;

            let out: Result<SchemaWortGramType, _> =
                stmt.query_one([], SchemaWortGramType::from_sql);

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
            let mut stmt = conn.prepare("SELECT 10, 3, '2025-12-04 20:00:00';")?;

            let out: Result<SchemaWortGramType, _> =
                stmt.query_one([], SchemaWortGramType::from_sql);

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
