use crate::db::traits::FromSql;

#[derive(Debug)]
pub struct SchemaWortGramType {
    pub id_worte: i32,
    pub id_gram_type: i32,

    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaWortGramType {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, crate::helpers::error_handler::DbError> {
        Ok(Self {
            id_worte: r.get(0)?,
            id_gram_type: r.get(1)?,

            created_at: r.get(2)?,
            deleted_at: r.get(3)?,
        })
    }
}

#[cfg(test)]
mod tests_schema_wort_gram_type_from_sql {
    use super::*;
    use color_eyre::Result;
    use rusqlite::{Connection, params};

    fn setup_conn() -> Result<Connection> {
        Ok(Connection::open_in_memory()?)
    }

    #[test]
    fn from_sql_ok_with_null_deleted_at() -> Result<()> {
        let conn = setup_conn()?;

        let out = conn.query_row(
            r#"
            SELECT
                ?1 as id_worte,
                ?2 as id_gram_type,
                ?3 as created_at,
                NULL as deleted_at
            "#,
            params![10i32, 4i32, "2025-12-01 00:00:00"],
            |row| SchemaWortGramType::from_sql(row),
        )?;

        assert_eq!(out.id_worte, 10);
        assert_eq!(out.id_gram_type, 4);
        assert_eq!(out.created_at, "2025-12-01 00:00:00");
        assert_eq!(out.deleted_at, None);

        Ok(())
    }

    #[test]
    fn from_sql_ok_with_deleted_at() -> Result<()> {
        let conn = setup_conn()?;

        let out = conn.query_row(
            r#"
            SELECT
                ?1 as id_worte,
                ?2 as id_gram_type,
                ?3 as created_at,
                ?4 as deleted_at
            "#,
            params![99i32, 12i32, "2025-12-01 00:00:00", "2025-12-31 23:59:59"],
            |row| SchemaWortGramType::from_sql(row),
        )?;

        assert_eq!(out.id_worte, 99);
        assert_eq!(out.id_gram_type, 12);
        assert_eq!(out.created_at, "2025-12-01 00:00:00");
        assert_eq!(out.deleted_at.as_deref(), Some("2025-12-31 23:59:59"));

        Ok(())
    }

    #[test]
    fn from_sql_err_when_missing_column() -> Result<()> {
        let conn = setup_conn()?;

        // deleted_at no existe (solo 3 columnas)
        let res = conn.query_row(
            r#"
            SELECT
                ?1 as id_worte,
                ?2 as id_gram_type,
                ?3 as created_at
            "#,
            params![1i32, 2i32, "2025-12-01 00:00:00"],
            |row| SchemaWortGramType::from_sql(row),
        );

        assert!(res.is_err(), "should fail due to missing column index 3");
        Ok(())
    }

    #[test]
    fn from_sql_err_when_type_mismatch() -> Result<()> {
        let conn = setup_conn()?;

        // id_worte viene como TEXT => r.get::<_, i32>(0) debe fallar
        let res = conn.query_row(
            r#"
            SELECT
                'not-an-int' as id_worte,
                ?1 as id_gram_type,
                ?2 as created_at,
                NULL as deleted_at
            "#,
            params![2i32, "2025-12-01 00:00:00"],
            |row| SchemaWortGramType::from_sql(row),
        );

        assert!(res.is_err(), "should fail due to i32 conversion");
        Ok(())
    }
}
