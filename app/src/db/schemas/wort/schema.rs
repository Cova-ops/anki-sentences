use crate::db::traits::FromSql;

#[derive(Debug, Clone)]
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
    /// Orden:
    /// - id
    /// - gender_id
    /// - worte_de
    /// - worte_es
    /// - plural
    /// - niveau_id
    /// - example_de
    /// - example_es
    /// - verb_aux
    /// - trennbar
    /// - reflexiv
    /// - created_at
    /// - deleted_at
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
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
    use super::*;

    use crate::helpers::error_handler::DbError;
    use rusqlite::Connection;

    mod from_sql {
        use super::*;

        #[test]
        fn ok_with_null_deleted_at() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            // Mix of Some/NULL for Option fields, deleted_at = NULL
            let mut stmt = conn.prepare(
                "SELECT
                    1,                      -- id
                    1,                      -- gender_id
                    'Haus',                 -- worte_de
                    'casa',                 -- worte_es
                    'Häuser',               -- plural
                    2,                      -- niveau_id
                    'Das Haus ist groß.',   -- example_de
                    'La casa es grande.',   -- example_es
                    NULL,                   -- verb_aux
                    NULL,                   -- trennbar
                    NULL,                   -- reflexiv
                    '2025-12-04 20:00:00',  -- created_at
                    NULL;                   -- deleted_at
                ",
            )?;

            let out: SchemaWort = stmt.query_one([], SchemaWort::from_sql)?;

            assert_eq!(out.id, 1);
            assert_eq!(out.gender_id, Some(1));
            assert_eq!(out.worte_de, "Haus");
            assert_eq!(out.worte_es, "casa");
            assert_eq!(out.plural.as_deref(), Some("Häuser"));
            assert_eq!(out.niveau_id, 2);
            assert_eq!(out.example_de, "Das Haus ist groß.");
            assert_eq!(out.example_es, "La casa es grande.");
            assert_eq!(out.verb_aux, None);
            assert_eq!(out.trennbar, None);
            assert_eq!(out.reflexiv, None);
            assert_eq!(out.created_at, "2025-12-04 20:00:00");
            assert_eq!(out.deleted_at, None);

            Ok(())
        }

        #[test]
        fn ok_with_some_deleted_at() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            // Verb-like row: verb_aux + booleans Some(...)
            let mut stmt = conn.prepare(
                "SELECT
                    2,                      -- id
                    NULL,                   -- gender_id
                    'aufstehen',            -- worte_de
                    'levantarse',           -- worte_es
                    NULL,                   -- plural
                    3,                      -- niveau_id
                    'Ich stehe um 7 Uhr auf.', -- example_de
                    'Me levanto a las 7.',   -- example_es
                    'sein',                 -- verb_aux
                    1,                      -- trennbar (true)
                    0,                      -- reflexiv (false)
                    '2025-12-04 20:00:00',  -- created_at
                    '2025-12-31 00:00:00';  -- deleted_at
                ",
            )?;

            let out: SchemaWort = stmt.query_one([], SchemaWort::from_sql)?;

            assert_eq!(out.id, 2);
            assert_eq!(out.gender_id, None);
            assert_eq!(out.worte_de, "aufstehen");
            assert_eq!(out.worte_es, "levantarse");
            assert_eq!(out.plural, None);
            assert_eq!(out.niveau_id, 3);
            assert_eq!(out.example_de, "Ich stehe um 7 Uhr auf.");
            assert_eq!(out.example_es, "Me levanto a las 7.");
            assert_eq!(out.verb_aux.as_deref(), Some("sein"));
            assert_eq!(out.trennbar, Some(true));
            assert_eq!(out.reflexiv, Some(false));
            assert_eq!(out.created_at, "2025-12-04 20:00:00");
            assert_eq!(out.deleted_at.as_deref(), Some("2025-12-31 00:00:00"));

            Ok(())
        }

        #[test]
        fn err_type_mismatch() -> Result<(), DbError> {
            let conn = Connection::open_in_memory()?;

            // niveau_id should be INTEGER (i32), but we provide TEXT
            let mut stmt = conn.prepare(
                "SELECT
                    1,
                    1,
                    'Haus',
                    'casa',
                    'Häuser',
                    'oops',                 -- niveau_id wrong type
                    'Das Haus ist groß.',
                    'La casa es grande.',
                    NULL,
                    NULL,
                    NULL,
                    '2025-12-04 20:00:00',
                    NULL;
                ",
            )?;

            let out: Result<SchemaWort, _> = stmt.query_one([], SchemaWort::from_sql);

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

            // deleted_at column missing (index 12)
            let mut stmt = conn.prepare(
                "SELECT
                    1,
                    1,
                    'Haus',
                    'casa',
                    'Häuser',
                    2,
                    'Das Haus ist groß.',
                    'La casa es grande.',
                    NULL,
                    NULL,
                    NULL,
                    '2025-12-04 20:00:00';
                ",
            )?;

            let out: Result<SchemaWort, _> = stmt.query_one([], SchemaWort::from_sql);

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
