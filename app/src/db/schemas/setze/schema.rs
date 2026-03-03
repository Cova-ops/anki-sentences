use crate::db::traits::FromSql;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaSetze {
    pub id: i32,

    pub setze_spanisch: String,
    pub setze_deutsch: String,
    pub niveau_id: i32,
    pub thema: String,

    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaSetze {
    /// Orden:
    /// - id
    /// - setze_spanisch
    /// - setze_deutsch
    /// - niveau_id
    /// - thema
    /// - created_at
    /// - deleted_at
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: r.get(0)?,
            setze_spanisch: r.get(1)?,
            setze_deutsch: r.get(2)?,
            niveau_id: r.get(3)?,
            thema: r.get(4)?,
            created_at: r.get(5)?,
            deleted_at: r.get(6)?,
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
                "SELECT 1, 'Hola mundo', 'Hallo Welt', 2, 'saludos', '2025-12-04 20:00:00', NULL;",
            )?;

            let out: SchemaSetze = stmt.query_one([], SchemaSetze::from_sql)?;

            assert_eq!(out.id, 1);
            assert_eq!(out.setze_spanisch, "Hola mundo");
            assert_eq!(out.setze_deutsch, "Hallo Welt");
            assert_eq!(out.niveau_id, 2);
            assert_eq!(out.thema, "saludos");
            assert_eq!(out.created_at, "2025-12-04 20:00:00");
            assert_eq!(out.deleted_at, None);

            Ok(())
        }

        #[test]
        fn ok_with_some_deleted_at() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            let mut stmt = conn.prepare(
                "SELECT 2, '¿Cómo estás?', 'Wie geht es dir?', 3, 'conversación', '2025-12-04 20:00:00', '2025-12-31 00:00:00';"
            )?;

            let out: SchemaSetze = stmt.query_one([], SchemaSetze::from_sql)?;

            assert_eq!(out.id, 2);
            assert_eq!(out.setze_spanisch, "¿Cómo estás?");
            assert_eq!(out.setze_deutsch, "Wie geht es dir?");
            assert_eq!(out.niveau_id, 3);
            assert_eq!(out.thema, "conversación");
            assert_eq!(out.created_at, "2025-12-04 20:00:00");
            assert_eq!(out.deleted_at.as_deref(), Some("2025-12-31 00:00:00"));

            Ok(())
        }

        #[test]
        fn err_type_mismatch() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            // id should be INTEGER but we provide TEXT
            let mut stmt = conn.prepare(
                "SELECT 'oops', 'Hola', 'Hallo', 2, 'tema', '2025-12-04 20:00:00', NULL;",
            )?;

            let out: Result<SchemaSetze, _> = stmt.query_one([], SchemaSetze::from_sql);

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
            let mut stmt = conn.prepare(
                "SELECT 1, 'Hola mundo', 'Hallo Welt', 2, 'saludos', '2025-12-04 20:00:00';",
            )?;

            let out: Result<SchemaSetze, _> = stmt.query_one([], SchemaSetze::from_sql);

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
