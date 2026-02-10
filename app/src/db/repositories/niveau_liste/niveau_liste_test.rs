#[cfg(test)]
mod tests_niveau_liste_repo_bulk_upsert {
    use crate::test_utils::prelude::*;
    use rusqlite::params;

    fn assert_matches_enum(rows: &[SnapshotNiveauListe]) {
        for r in rows {
            let niveau = EnumNiveauListe::try_from(r.niveau.id()).unwrap_or_else(|| {
                panic!("unknown niveau_liste id in DB snapshot: {}", r.niveau.id())
            });

            // Ajusta estas 2 líneas si tu enum usa otros getters
            assert_eq!(
                r.niveau,
                niveau.to_code(),
                "niveau mismatch for id={}",
                r.niveau.id()
            );
        }
    }

    mod bulk_upsert {
        use super::*;

        #[test]
        fn bulk_upsert_behaviour() -> Result<(), DbError> {
            let mut conn = setup_test_db()?;

            // 1) Empty
            let res = NiveauListeRepo::bulk_upsert(&mut conn, &[])?;
            let snapshot: Vec<SnapshotNiveauListe> = res.into_iter().map(Into::into).collect();

            assert!(snapshot.is_empty());
            insta::assert_debug_snapshot!("[NiveauListe::bulk_upsert] - empty", snapshot);

            // 2) Normal insert
            let sc = scenario_niveau_liste();
            let res = NiveauListeRepo::bulk_upsert(&mut conn, &sc.initial)?;
            let snapshot: Vec<SnapshotNiveauListe> = res.into_iter().map(Into::into).collect();

            assert_eq!(snapshot.len(), sc.initial.len());
            assert_matches_enum(&snapshot);
            insta::assert_debug_snapshot!("[NiveauListe::bulk_upsert] - after insert", snapshot);

            // 3) Force corruption and ensure upsert repairs via conflict(id)
            {
                let tx = conn.transaction()?;
                DbQuery::execute(
                    &tx,
                    r#"
                UPDATE niveau_liste
                SET niveau = ?2
                WHERE id = ?1
                "#,
                    params![0i32, "WRONG_NIVEAU"],
                )?;
                tx.commit()?;
            }

            let res = NiveauListeRepo::bulk_upsert(&mut conn, &sc.initial)?;
            let snapshot: Vec<SnapshotNiveauListe> = res.into_iter().map(Into::into).collect();

            assert_eq!(snapshot.len(), sc.initial.len());
            assert_matches_enum(&snapshot);
            insta::assert_debug_snapshot!(
                "[NiveauListe::bulk_upsert] - after conflict(id) repair",
                snapshot
            );

            Ok(())
        }
    }
}
