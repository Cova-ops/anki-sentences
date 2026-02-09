use crate::db::traits::FromSql;

#[derive(Debug)]
pub struct SchemaWortAudio {
    pub wort_id: i32,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,

    // Generic
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaWortAudio {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, crate::helpers::error_handler::DbError> {
        Ok(Self {
            wort_id: r.get(0)?,
            audio_name_es: r.get(1)?,
            audio_name_de: r.get(2)?,
            created_at: r.get(3)?,
            deleted_at: r.get(4)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{db::queries::DbQuery, helpers::error_handler::DbError};

    use super::*;
    use rusqlite::Connection;

    fn setup_db() -> Result<Connection, DbError> {
        let mut conn = Connection::open_in_memory()?;
        let tx = conn.transaction()?;

        DbQuery::execute(
            &tx,
            r#"
            CREATE TABLE worte_audio (
                wort_id INTEGER NOT NULL,
                audio_name_es TEXT,
                audio_name_de TEXT,
                created_at TEXT NOT NULL,
                deleted_at TEXT
            );
            "#,
            [],
        )?;

        tx.commit()?;
        Ok(conn)
    }

    #[test]
    fn from_sql_ok_with_all_fields() -> Result<(), DbError> {
        let mut conn = setup_db()?;
        let tx = conn.transaction()?;

        DbQuery::execute(
            &tx,
            r#"
            INSERT INTO worte_audio (
                wort_id,
                audio_name_es,
                audio_name_de,
                created_at,
                deleted_at
            ) VALUES (?1, ?2, ?3, ?4, ?5);
            "#,
            (
                10,
                Some("audio_es.mp3"),
                Some("audio_de.mp3"),
                "2025-01-01 10:00:00",
                Some("2025-01-02 10:00:00"),
            ),
        )?;

        let schema: SchemaWortAudio = DbQuery::query_one(
            &tx,
            r#"
            SELECT wort_id, audio_name_es, audio_name_de, created_at, deleted_at
            FROM worte_audio
            "#,
            [],
            SchemaWortAudio::from_sql,
        )?;

        tx.commit()?;

        assert_eq!(schema.wort_id, 10);
        assert_eq!(schema.audio_name_es.as_deref(), Some("audio_es.mp3"));
        assert_eq!(schema.audio_name_de.as_deref(), Some("audio_de.mp3"));
        assert_eq!(schema.created_at, "2025-01-01 10:00:00".to_string());
        assert_eq!(schema.deleted_at.as_deref(), Some("2025-01-02 10:00:00"));

        Ok(())
    }

    #[test]
    fn from_sql_ok_with_null_optionals() -> Result<(), DbError> {
        let mut conn = setup_db()?;
        let tx = conn.transaction()?;

        DbQuery::execute(
            &tx,
            r#"
            INSERT INTO worte_audio (
                wort_id,
                audio_name_es,
                audio_name_de,
                created_at,
                deleted_at
            ) VALUES (?1, ?2, ?3, ?4, ?5);
            "#,
            (
                5,
                None::<String>,
                None::<String>,
                "2025-01-01 12:00:00",
                None::<String>,
            ),
        )?;

        let schema: SchemaWortAudio = DbQuery::query_one(
            &tx,
            r#"
            SELECT wort_id, audio_name_es, audio_name_de, created_at, deleted_at
            FROM worte_audio
            "#,
            [],
            SchemaWortAudio::from_sql,
        )?;

        tx.commit()?;

        assert_eq!(schema.wort_id, 5);
        assert_eq!(schema.audio_name_es, None);
        assert_eq!(schema.audio_name_de, None);
        assert_eq!(schema.created_at, "2025-01-01 12:00:00".to_string());
        assert_eq!(schema.deleted_at, None);

        Ok(())
    }
}
