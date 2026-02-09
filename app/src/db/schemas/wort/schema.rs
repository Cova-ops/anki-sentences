use crate::db::traits::FromSql;

#[derive(Debug)]

pub struct SchemaWort {
    pub id: i32,
    pub gender_id: Option<i32>,
    pub worte_de: String,
    pub worte_es: String,
    pub plural: Option<String>,
    pub niveau_id: i32,
    pub example_de: String,
    pub example_es: String,

    // nur verben
    pub verb_aux: Option<String>,
    pub trennbar: Option<bool>,
    pub reflexiv: Option<bool>,

    // Generic
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaWort {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, crate::helpers::error_handler::DbError> {
        Ok(Self {
            id: r.get(0)?,
            gender_id: r.get(1)?,
            worte_de: r.get(2)?,
            worte_es: r.get(3)?,
            plural: r.get(4)?,
            niveau_id: r.get(5)?,
            example_de: r.get(6)?,
            example_es: r.get(7)?,
            verb_aux: r.get(8)?,
            trennbar: r.get(9)?,
            reflexiv: r.get(10)?,
            created_at: r.get(11)?,
            deleted_at: r.get(12)?,
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
            CREATE TABLE wort (
                id INTEGER,
                gender_id INTEGER,
                worte_de TEXT NOT NULL,
                worte_es TEXT NOT NULL,
                plural TEXT,
                niveau_id INTEGER NOT NULL,
                example_de TEXT NOT NULL,
                example_es TEXT NOT NULL,
                verb_aux TEXT,
                trennbar BOOLEAN,
                reflexiv BOOLEAN,
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
    fn schema_wort_from_sql_full_row() -> Result<(), DbError> {
        let mut conn = setup_db()?;
        let tx = conn.transaction()?;

        DbQuery::execute(
            &tx,
            r#"
            INSERT INTO wort VALUES (
                1,
                0,
                'Haus',
                'Casa',
                'Häuser',
                2,
                'Das Haus ist groß.',
                'La casa es grande.',
                'sein',
                1,
                0,
                '2025-12-04 17:44:37',
                NULL
            );
            "#,
            [],
        )?;

        let schema: SchemaWort =
            DbQuery::query_one(&tx, "SELECT * FROM wort", [], SchemaWort::from_sql)?;

        tx.commit()?;

        assert_eq!(schema.id, 1);
        assert_eq!(schema.gender_id, Some(0));
        assert_eq!(schema.worte_de, "Haus");
        assert_eq!(schema.worte_es, "Casa");
        assert_eq!(schema.plural.as_deref(), Some("Häuser"));
        assert_eq!(schema.niveau_id, 2);
        assert_eq!(schema.example_de, "Das Haus ist groß.");
        assert_eq!(schema.example_es, "La casa es grande.");
        assert_eq!(schema.verb_aux.as_deref(), Some("sein"));
        assert_eq!(schema.trennbar, Some(true));
        assert_eq!(schema.reflexiv, Some(false));
        assert_eq!(schema.created_at, "2025-12-04 17:44:37");
        assert_eq!(schema.deleted_at, None);

        Ok(())
    }

    #[test]
    fn schema_wort_from_sql_with_null_optionals() -> Result<(), DbError> {
        let mut conn = setup_db()?;
        let tx = conn.transaction()?;

        DbQuery::execute(
            &tx,
            r#"
            INSERT INTO wort VALUES (
                2,
                NULL,
                'gehen',
                'ir',
                NULL,
                1,
                'Ich gehe nach Hause.',
                'Voy a casa.',
                NULL,
                NULL,
                NULL,
                '2025-12-04 18:00:00',
                '2025-12-05 10:00:00'
            );
            "#,
            [],
        )?;

        let schema: SchemaWort =
            DbQuery::query_one(&tx, "SELECT * FROM wort", [], SchemaWort::from_sql)?;

        tx.commit()?;

        assert_eq!(schema.id, 2);
        assert_eq!(schema.gender_id, None);
        assert_eq!(schema.plural, None);
        assert_eq!(schema.verb_aux, None);
        assert_eq!(schema.trennbar, None);
        assert_eq!(schema.reflexiv, None);
        assert_eq!(schema.deleted_at.as_deref(), Some("2025-12-05 10:00:00"));

        Ok(())
    }
}
