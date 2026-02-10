use crate::db::traits::FromSql;

#[derive(Debug)]
pub struct SchemaNiveauListe {
    pub niveau: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaNiveauListe {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            niveau: r.get(0)?,
            created_at: r.get(1)?,
            deleted_at: r.get(2)?,
        })
    }
}

#[cfg(test)]
mod tests_schema_niveau_liste_from_sql {
    use super::*;

    use crate::helpers::error_handler::DbError;
    use rusqlite::{Connection, params};

    fn setup_conn() -> Result<Connection, DbError> {
        let conn = Connection::open_in_memory()?;

        conn.execute(
            r#"
            CREATE TABLE niveau_liste (
                niveau     TEXT NOT NULL,
                created_at TEXT NOT NULL,
                deleted_at TEXT
            );
            "#,
            [],
        )?;

        Ok(conn)
    }

    #[test]
    fn from_sql_ok_with_null_deleted_at() -> Result<(), DbError> {
        let conn = setup_conn()?;

        conn.execute(
            "INSERT INTO niveau_liste (niveau, created_at, deleted_at) VALUES (?1, ?2, NULL)",
            params!["A2", "2025-12-04 18:07:37"],
        )?;

        let sql = "SELECT niveau, created_at, deleted_at FROM niveau_liste LIMIT 1";
        let out: SchemaNiveauListe = conn.query_one(sql, [], SchemaNiveauListe::from_sql)?;

        assert_eq!(out.niveau, "A2");
        assert_eq!(out.created_at, "2025-12-04 18:07:37");
        assert_eq!(out.deleted_at, None);

        Ok(())
    }

    #[test]
    fn from_sql_ok_with_some_deleted_at() -> Result<(), DbError> {
        let conn = setup_conn()?;

        conn.execute(
            "INSERT INTO niveau_liste (niveau, created_at, deleted_at) VALUES (?1, ?2, ?3)",
            params!["B1", "2025-12-04 18:07:37", "2025-12-31 00:00:00"],
        )?;

        let sql = "SELECT niveau, created_at, deleted_at FROM niveau_liste LIMIT 1";
        let out: SchemaNiveauListe = conn.query_one(sql, [], SchemaNiveauListe::from_sql)?;

        assert_eq!(out.niveau, "B1");
        assert_eq!(out.created_at, "2025-12-04 18:07:37");
        assert_eq!(out.deleted_at.as_deref(), Some("2025-12-31 00:00:00"));

        Ok(())
    }

    #[test]
    fn from_sql_err_when_type_mismatch() -> Result<(), DbError> {
        let conn = setup_conn()?;

        // nivel como INTEGER (mal) para forzar error al leer como String
        conn.execute("DELETE FROM niveau_liste;", [])?;
        conn.execute(
            r#"
            INSERT INTO niveau_liste (niveau, created_at, deleted_at)
            VALUES (123, '2025-12-04 18:07:37', NULL);
            "#,
            [],
        )?;

        let sql = "SELECT niveau, created_at, deleted_at FROM niveau_liste LIMIT 1";
        let res: Result<SchemaNiveauListe, _> =
            conn.query_one(sql, [], SchemaNiveauListe::from_sql);

        // no commit: este test espera error
        assert!(res.is_err());
        let err = res.unwrap_err();

        // Validamos que exista error y que venga de algo de type mismatch
        let msg = format!("{err:?}");
        assert!(
            msg.contains("InvalidColumnType") || msg.contains("type") || msg.contains("column"),
            "Unexpected error: {msg}"
        );

        Ok(())
    }

    #[test]
    fn from_sql_err_when_missing_column() -> Result<(), DbError> {
        let conn = setup_conn()?;

        conn.execute(
            "INSERT INTO niveau_liste (niveau, created_at, deleted_at) VALUES (?1, ?2, NULL)",
            params!["C1", "2025-12-04 18:07:37"],
        )?;

        // OJO: aquí seleccionamos SOLO 2 columnas pero from_sql intenta get(2)
        let sql = "SELECT niveau, created_at FROM niveau_liste LIMIT 1";
        let res: Result<SchemaNiveauListe, _> =
            conn.query_one(sql, [], SchemaNiveauListe::from_sql);

        assert!(res.is_err());
        let err = res.unwrap_err();

        let msg = format!("{err:?}");
        assert!(
            msg.contains("InvalidColumnIndex") || msg.contains("column") || msg.contains("index"),
            "Unexpected error: {msg}"
        );

        Ok(())
    }
}
