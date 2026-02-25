#[cfg(test)]
mod tests_niveau_liste_repo_bulk_upsert {
    use rusqlite::{Connection, params};

    use crate::db::schemas::{
        init_schemas,
        niveau_liste::{InputNiveauListe, SchemaNiveauListe, SnapshotNiveauListe},
    };
    use crate::{
        db::niveau_liste::NiveauListeRepo, helpers::error_handler::DbError,
        test_utils::scenarios::scenario_niveau_liste,
    };

    fn assert_iter(res: &[SchemaNiveauListe], data: &[InputNiveauListe]) {
        assert_eq!(res.len(), data.len());

        for (i, satz) in data.iter().enumerate() {
            assert_eq!(res[i].niveau, satz.niveau.as_str());
            assert!(res[i].deleted_at.is_none());
        }
    }

    mod bulk_upsert {

        use super::*;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let res = NiveauListeRepo::bulk_upsert(&mut conn, &[])?;

            assert_iter(&res, &[]);

            let snapshot: Vec<SnapshotNiveauListe> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[NiveauListe::bulk_upsert] - empty", snapshot);

            Ok(())
        }

        #[test]
        fn insert() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_niveau_liste().initial;
            let res = NiveauListeRepo::bulk_upsert(&mut conn, &data)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotNiveauListe> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[NiveauListe::bulk_upsert] - insert", snapshot);

            Ok(())
        }

        #[test]
        fn update() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_niveau_liste().initial;
            NiveauListeRepo::bulk_upsert(&mut conn, &data)?;

            // Force corruption and ensure upsert repairs via conflict(id)
            {
                conn.execute(
                    r#"
                UPDATE niveau_liste
                SET niveau = ?2
                WHERE id = ?1
                "#,
                    params![0i32, "WRONG_NIVEAU"],
                )?;
            }

            let res = NiveauListeRepo::bulk_upsert(&mut conn, &data)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotNiveauListe> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[NiveauListe::bulk_upsert] - update", snapshot);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let data = scenario_niveau_liste().initial;
            let err = NiveauListeRepo::bulk_upsert(&mut conn, &data).unwrap_err();

            assert!(err.sql.is_some(), "expected DbError.sql to be Some(sql)");
            assert!(
                err.message.to_lowercase().contains("no such table")
                    || format!("{:?}", err)
                        .to_lowercase()
                        .contains("no such table"),
                "unexpected error: {err:?}"
            );

            Ok(())
        }
    }
}
