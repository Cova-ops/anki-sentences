use crate::db::traits::FromSql;

#[derive(Debug)]
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
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, crate::helpers::error_handler::DbError> {
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
mod tests_schema_setze {
    use super::*;
    use rusqlite::{Connection, params};

    #[test]
    fn from_sql_maps_all_fields_correctly() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute_batch(
            r#"
            CREATE TABLE setze (
                id INTEGER,
                setze_spanisch TEXT,
                setze_deutsch TEXT,
                niveau_id INTEGER,
                thema TEXT,
                created_at TEXT,
                deleted_at TEXT
            );
            "#,
        )
        .unwrap();

        conn.execute(
            r#"
            INSERT INTO setze (
                id, setze_spanisch, setze_deutsch, niveau_id, thema, created_at, deleted_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            params![
                1,
                "Estoy aprendiendo alemán.",
                "Ich lerne Deutsch.",
                2,
                "learning",
                "2025-01-01 10:00:00",
                Option::<String>::None
            ],
        )
        .unwrap();

        let mut stmt = conn
            .prepare(
                r#"
                SELECT
                    id,
                    setze_spanisch,
                    setze_deutsch,
                    niveau_id,
                    thema,
                    created_at,
                    deleted_at
                FROM setze
                "#,
            )
            .unwrap();

        let schema = stmt
            .query_row([], |row| SchemaSetze::from_sql(row))
            .unwrap();

        assert_eq!(schema.id, 1);
        assert_eq!(schema.setze_spanisch, "Estoy aprendiendo alemán.");
        assert_eq!(schema.setze_deutsch, "Ich lerne Deutsch.");
        assert_eq!(schema.niveau_id, 2);
        assert_eq!(schema.thema, "learning");
        assert_eq!(schema.created_at, "2025-01-01 10:00:00");
        assert_eq!(schema.deleted_at, None);
    }

    #[test]
    fn from_sql_with_deleted_at() {
        let conn = Connection::open_in_memory().unwrap();

        conn.execute_batch(
            r#"
            CREATE TABLE setze (
                id INTEGER,
                setze_spanisch TEXT,
                setze_deutsch TEXT,
                niveau_id INTEGER,
                thema TEXT,
                created_at TEXT,
                deleted_at TEXT
            );
            "#,
        )
        .unwrap();

        conn.execute(
            r#"
            INSERT INTO setze VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
            params![
                2,
                "Ella trabaja aquí.",
                "Sie arbeitet hier.",
                3,
                "work",
                "2025-01-02 12:00:00",
                "2025-02-01 00:00:00"
            ],
        )
        .unwrap();

        let schema: SchemaSetze = conn
            .query_row("SELECT * FROM setze", [], |row| SchemaSetze::from_sql(row))
            .unwrap();

        assert_eq!(schema.id, 2);
        assert_eq!(schema.deleted_at, Some("2025-02-01 00:00:00".into()));
    }
}
