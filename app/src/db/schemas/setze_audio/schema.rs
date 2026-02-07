use crate::db::traits::FromSql;

#[derive(Debug)]
pub struct SchemaSetzeAudio {
    pub satz_id: i32,
    pub file_path: String,
    pub voice_id: String,

    // Generic
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaSetzeAudio {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, crate::helpers::error_handler::DbError> {
        Ok(Self {
            satz_id: r.get(0)?,
            file_path: r.get(1)?,
            voice_id: r.get(2)?,
            created_at: r.get(3)?,
            deleted_at: r.get(4)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{Connection, params};

    #[test]
    fn from_sql_maps_row_correctly() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            r#"
            CREATE TABLE setze_audio (
                satz_id     INTEGER NOT NULL,
                file_path   TEXT NOT NULL,
                voice_id    TEXT NOT NULL,
                created_at  TEXT NOT NULL,
                deleted_at  TEXT
            );
            "#,
            [],
        )
        .unwrap();

        conn.execute(
            r#"
            INSERT INTO setze_audio
                (satz_id, file_path, voice_id, created_at, deleted_at)
            VALUES
                (?1, ?2, ?3, ?4, ?5);
            "#,
            params![
                10,
                "audios/setze/10.mp3",
                "voice_de",
                "2025-01-01 12:00:00",
                Option::<String>::None
            ],
        )
        .unwrap();

        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    satz_id,
                    file_path,
                    voice_id,
                    created_at,
                    deleted_at
                FROM setze_audio;
                "#,
            )
            .unwrap();

        let schema = stmt
            .query_row([], |row| SchemaSetzeAudio::from_sql(row))
            .unwrap();

        assert_eq!(schema.satz_id, 10);
        assert_eq!(schema.file_path, "audios/setze/10.mp3");
        assert_eq!(schema.voice_id, "voice_de");
        assert_eq!(schema.created_at, "2025-01-01 12:00:00");
        assert_eq!(schema.deleted_at, None);
    }

    #[test]
    fn from_sql_with_deleted_at() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute(
            r#"
            CREATE TABLE setze_audio (
                satz_id     INTEGER NOT NULL,
                file_path   TEXT NOT NULL,
                voice_id    TEXT NOT NULL,
                created_at  TEXT NOT NULL,
                deleted_at  TEXT
            );
            "#,
            [],
        )
        .unwrap();

        conn.execute(
            r#"
            INSERT INTO setze_audio
                (satz_id, file_path, voice_id, created_at, deleted_at)
            VALUES
                (?1, ?2, ?3, ?4, ?5);
            "#,
            params![
                11,
                "audios/setze/11.mp3",
                "voice_es",
                "2025-01-02 10:00:00",
                "2025-01-10 00:00:00"
            ],
        )
        .unwrap();

        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    satz_id,
                    file_path,
                    voice_id,
                    created_at,
                    deleted_at
                FROM setze_audio;
                "#,
            )
            .unwrap();

        let schema = stmt
            .query_row([], |row| SchemaSetzeAudio::from_sql(row))
            .unwrap();

        assert_eq!(schema.satz_id, 11);
        assert_eq!(schema.file_path, "audios/setze/11.mp3");
        assert_eq!(schema.voice_id, "voice_es");
        assert_eq!(schema.created_at, "2025-01-02 10:00:00");
        assert_eq!(schema.deleted_at, Some("2025-01-10 00:00:00".to_string()));
    }
}
