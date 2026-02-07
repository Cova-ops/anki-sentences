use crate::db::traits::FromSql;

#[derive(Debug)]
pub struct SchemaNiveauListe {
    pub niveau: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
}

impl FromSql for SchemaNiveauListe {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, crate::helpers::error_handler::DbError> {
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
    use rusqlite::Connection;

    fn setup_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            r#"
            CREATE TABLE niveau_liste (
                niveau     TEXT NOT NULL,
                created_at TEXT NOT NULL,
                deleted_at TEXT
            );
            "#,
        )
        .unwrap();
        conn
    }

    #[test]
    fn from_sql_ok_with_null_deleted_at() {
        let conn = setup_conn();

        conn.execute(
            "INSERT INTO niveau_liste (niveau, created_at, deleted_at) VALUES (?1, ?2, NULL)",
            ("A2", "2025-12-04 18:07:37"),
        )
        .unwrap();

        let sql = "SELECT niveau, created_at, deleted_at FROM niveau_liste LIMIT 1";
        let mut stmt = conn.prepare(sql).unwrap();

        let out: SchemaNiveauListe = stmt
            .query_row([], |row| SchemaNiveauListe::from_sql(row))
            .unwrap();

        assert_eq!(out.niveau, "A2");
        assert_eq!(out.created_at, "2025-12-04 18:07:37");
        assert_eq!(out.deleted_at, None);
    }

    #[test]
    fn from_sql_ok_with_some_deleted_at() {
        let conn = setup_conn();

        conn.execute(
            "INSERT INTO niveau_liste (niveau, created_at, deleted_at) VALUES (?1, ?2, ?3)",
            ("B1", "2025-12-04 18:07:37", "2025-12-31 00:00:00"),
        )
        .unwrap();

        let sql = "SELECT niveau, created_at, deleted_at FROM niveau_liste LIMIT 1";
        let mut stmt = conn.prepare(sql).unwrap();

        let out: SchemaNiveauListe = stmt
            .query_row([], |row| SchemaNiveauListe::from_sql(row))
            .unwrap();

        assert_eq!(out.niveau, "B1");
        assert_eq!(out.created_at, "2025-12-04 18:07:37");
        assert_eq!(out.deleted_at.as_deref(), Some("2025-12-31 00:00:00"));
    }

    #[test]
    fn from_sql_err_when_type_mismatch() {
        let conn = setup_conn();

        // nivel como INTEGER (mal) para forzar error al leer como String
        conn.execute_batch(
            r#"
            DELETE FROM niveau_liste;
            INSERT INTO niveau_liste (niveau, created_at, deleted_at)
            VALUES (123, '2025-12-04 18:07:37', NULL);
            "#,
        )
        .unwrap();

        let sql = "SELECT niveau, created_at, deleted_at FROM niveau_liste LIMIT 1";
        let mut stmt = conn.prepare(sql).unwrap();

        let err = stmt
            .query_row([], |row| SchemaNiveauListe::from_sql(row))
            .unwrap_err();

        // depende de tu DbError, aquí solo validamos que el error exista
        // y que venga de rusqlite (type mismatch)
        let msg = format!("{err:?}");
        assert!(
            msg.contains("InvalidColumnType") || msg.contains("type") || msg.contains("column"),
            "Unexpected error: {msg}"
        );
    }

    #[test]
    fn from_sql_err_when_missing_column() {
        let conn = setup_conn();

        conn.execute(
            "INSERT INTO niveau_liste (niveau, created_at, deleted_at) VALUES (?1, ?2, NULL)",
            ("C1", "2025-12-04 18:07:37"),
        )
        .unwrap();

        // OJO: aquí seleccionamos SOLO 2 columnas pero from_sql intenta get(2)
        let sql = "SELECT niveau, created_at FROM niveau_liste LIMIT 1";
        let mut stmt = conn.prepare(sql).unwrap();

        let err = stmt
            .query_row([], |row| SchemaNiveauListe::from_sql(row))
            .unwrap_err();

        let msg = format!("{err:?}");
        assert!(
            msg.contains("InvalidColumnIndex") || msg.contains("column") || msg.contains("index"),
            "Unexpected error: {msg}"
        );
    }
}
