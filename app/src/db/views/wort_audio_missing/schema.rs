use crate::db::traits::FromSql;

#[derive(Debug, PartialEq, Eq)]
pub struct SchemaWortAudioMissing {
    pub id: i32,
    pub wort_es: String,
    pub wort_de: String,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,
}

impl FromSql for SchemaWortAudioMissing {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: r.get(0)?,
            wort_es: r.get(1)?,
            wort_de: r.get(2)?,
            audio_name_es: r.get(3)?,
            audio_name_de: r.get(4)?,
        })
    }
}

#[cfg(test)]
mod tests_schema_wort_audio_missing {
    use rusqlite::Connection;

    use crate::{
        db::{traits::FromSql, views::wort_audio_missing::SchemaWortAudioMissing},
        helpers::error_handler::DbError,
    };

    fn setup_db() -> Result<Connection, DbError> {
        let conn = Connection::open_in_memory()?;

        conn.execute(
            r#"
            CREATE TABLE wort_audio_missing (
                id              INTEGER NOT NULL,
                wort_es         TEXT NOT NULL,
                wort_de         TEXT NOT NULL,
                audio_name_es   TEXT,
                audio_name_de   TEXT
            );
            "#,
            [],
        )?;

        Ok(conn)
    }

    #[test]
    fn from_sql_with_all_fields() -> Result<(), DbError> {
        let conn = setup_db()?;

        conn.execute(
            r#"
            INSERT INTO wort_audio_missing
            (id, wort_es, wort_de, audio_name_es, audio_name_de)
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            (1, "hola", "hallo", Some("hola.mp3"), Some("hallo.mp3")),
        )?;

        let row: SchemaWortAudioMissing = conn.query_one(
            r#"
            SELECT id, wort_es, wort_de, audio_name_es, audio_name_de
            FROM wort_audio_missing
            "#,
            [],
            SchemaWortAudioMissing::from_sql,
        )?;

        assert_eq!(row.id, 1);
        assert_eq!(row.wort_es, "hola");
        assert_eq!(row.wort_de, "hallo");
        assert_eq!(row.audio_name_es, Some("hola.mp3".to_string()));
        assert_eq!(row.audio_name_de, Some("hallo.mp3".to_string()));

        Ok(())
    }

    #[test]
    fn from_sql_with_null_audio_fields() -> Result<(), DbError> {
        let conn = setup_db()?;

        conn.execute(
            r#"
            INSERT INTO wort_audio_missing
            (id, wort_es, wort_de, audio_name_es, audio_name_de)
            VALUES (?1, ?2, ?3, NULL, NULL)
            "#,
            (2, "gracias", "danke"),
        )?;

        let row: SchemaWortAudioMissing = conn.query_one(
            r#"
            SELECT id, wort_es, wort_de, audio_name_es, audio_name_de
            FROM wort_audio_missing
            "#,
            [],
            SchemaWortAudioMissing::from_sql,
        )?;

        assert_eq!(row.id, 2);
        assert_eq!(row.wort_es, "gracias");
        assert_eq!(row.wort_de, "danke");
        assert!(row.audio_name_es.is_none());
        assert!(row.audio_name_de.is_none());

        Ok(())
    }
}
