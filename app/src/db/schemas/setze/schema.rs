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
    use crate::{db::queries::DbQuery, helpers::error_handler::DbError};

    use super::*;
    use rusqlite::{Connection, params};

    fn setup_conn() -> Result<Connection, DbError> {
        let mut conn = Connection::open_in_memory()?;
        let tx = conn.transaction()?;

        DbQuery::execute(
            &tx,
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
            [],
        )?;

        tx.commit()?;
        Ok(conn)
    }

    #[test]
    fn from_sql_maps_all_fields_correctly() -> Result<(), DbError> {
        let mut conn = setup_conn()?;
        let tx = conn.transaction()?;

        DbQuery::execute(
            &tx,
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
        )?;

        let sql = r#"
            SELECT
                id,
                setze_spanisch,
                setze_deutsch,
                niveau_id,
                thema,
                created_at,
                deleted_at
            FROM setze
        "#;

        let schema: SchemaSetze = DbQuery::query_one(&tx, sql, [], SchemaSetze::from_sql)?;

        tx.commit()?;

        assert_eq!(schema.id, 1);
        assert_eq!(schema.setze_spanisch, "Estoy aprendiendo alemán.");
        assert_eq!(schema.setze_deutsch, "Ich lerne Deutsch.");
        assert_eq!(schema.niveau_id, 2);
        assert_eq!(schema.thema, "learning");
        assert_eq!(schema.created_at, "2025-01-01 10:00:00");
        assert_eq!(schema.deleted_at, None);

        Ok(())
    }

    #[test]
    fn from_sql_with_deleted_at() -> Result<(), DbError> {
        let mut conn = setup_conn()?;
        let tx = conn.transaction()?;

        DbQuery::execute(
            &tx,
            r#"
            INSERT INTO setze (
                id, setze_spanisch, setze_deutsch, niveau_id, thema, created_at, deleted_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
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
        )?;

        let schema: SchemaSetze =
            DbQuery::query_one(&tx, "SELECT * FROM setze", [], SchemaSetze::from_sql)?;

        tx.commit()?;

        assert_eq!(schema.id, 2);
        assert_eq!(schema.deleted_at.as_deref(), Some("2025-02-01 00:00:00"));

        Ok(())
    }
}
