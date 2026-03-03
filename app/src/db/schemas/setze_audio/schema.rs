use crate::db::traits::FromSql;

#[derive(Debug)]
pub struct SchemaSetzeAudio {
    pub satz_id: i32,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,

    // Generic
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaSetzeAudio {
    /// Orden:
    /// - satz_id
    /// - audio_name_es
    /// - audio_name_de
    /// - created_at
    /// - deleted_at
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            satz_id: r.get(0)?,
            audio_name_es: r.get(1)?,
            audio_name_de: r.get(2)?,
            created_at: r.get(3)?,
            deleted_at: r.get(4)?,
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

            let mut stmt =
                conn.prepare("SELECT 5, 'casa.mp3', NULL, '2025-12-04 20:00:00', NULL; ")?;

            let out: SchemaSetzeAudio = stmt.query_one([], SchemaSetzeAudio::from_sql)?;

            assert_eq!(out.satz_id, 5);
            assert_eq!(out.audio_name_es.as_deref(), Some("casa.mp3"));
            assert_eq!(out.audio_name_de, None);
            assert_eq!(out.created_at, "2025-12-04 20:00:00");
            assert_eq!(out.deleted_at, None);

            Ok(())
        }

        #[test]
        fn ok_with_some_deleted_at() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            let mut stmt = conn.prepare("SELECT 6, 'haus_es.mp3', 'haus_de.mp3', '2025-12-04 20:00:00', '2025-12-31 00:00:00';")?;

            let out: SchemaSetzeAudio = stmt.query_one([], SchemaSetzeAudio::from_sql)?;

            assert_eq!(out.satz_id, 6);
            assert_eq!(out.audio_name_es.as_deref(), Some("haus_es.mp3"));
            assert_eq!(out.audio_name_de.as_deref(), Some("haus_de.mp3"));
            assert_eq!(out.created_at, "2025-12-04 20:00:00");
            assert_eq!(out.deleted_at.as_deref(), Some("2025-12-31 00:00:00"));

            Ok(())
        }

        #[test]
        fn err_type_mismatch() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            // satz_id should be INTEGER but we pass TEXT
            let mut stmt =
                conn.prepare("SELECT 'wrong', 'casa.mp3', NULL, '2025-12-04 20:00:00', NULL;")?;

            let out: Result<SchemaSetzeAudio, _> = stmt.query_one([], SchemaSetzeAudio::from_sql);

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
            let mut stmt = conn.prepare("SELECT 5, 'casa.mp3', NULL, '2025-12-04 20:00:00';")?;

            let out: Result<SchemaSetzeAudio, _> = stmt.query_one([], SchemaSetzeAudio::from_sql);

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
