#[cfg(test)]
mod tests_gram_type_repo_bulk_upsert {
    use crate::{db::schemas::gram_type::SnapshotGramType, test_utils::prelude::*};
    use rusqlite::params;

    fn assert_matches_enum(rows: &[SnapshotGramType]) {
        for r in rows {
            let gram = EnumGramType::try_from(r.gram.id())
                .unwrap_or_else(|_| panic!("unknown gram_type id in DB snapshot: {}", r.gram.id()));

            assert_eq!(
                r.gram.to_code(),
                gram.to_code(),
                "code mismatch for id={}",
                r.gram.id()
            );
            assert_eq!(
                r.gram.to_name(),
                gram.to_name(),
                "name mismatch for id={}",
                r.gram.id()
            );
        }
    }

    #[test]
    fn bulk_upsert_behaviour() -> Result<(), DbError> {
        let mut conn = setup_test_db()?;

        // 1) Empty
        let res = GramTypeRepo::bulk_upsert(&mut conn, &[])?;
        let snapshot: Vec<SnapshotGramType> = res.into_iter().map(Into::into).collect();

        assert!(snapshot.is_empty());
        insta::assert_debug_snapshot!("[GramType::bulk_upsert] - empty", snapshot);

        // 2) Normal insert
        let sc = scenario_gram_type();
        let res = GramTypeRepo::bulk_upsert(&mut conn, &sc.initial)?;
        let snapshot: Vec<SnapshotGramType> = res.into_iter().map(Into::into).collect();

        assert_eq!(snapshot.len(), sc.initial.len());
        assert_matches_enum(&snapshot);
        insta::assert_debug_snapshot!("[GramType::bulk_upsert] - after insert", snapshot);

        // 3) Force UPDATE via conflict(id): same id, wrong code/name
        {
            let tx = conn.transaction()?;
            DbQuery::execute(
                &tx,
                r#"
                UPDATE gram_type
                SET code = ?2, name = ?3
                WHERE id = ?1
                "#,
                params![0i32, "WRONG_CODE", "WRONG_NAME"],
            )?;
            tx.commit()?;
        }

        // calling upsert again should correct the row back to enum-derived code/name
        let res = GramTypeRepo::bulk_upsert(&mut conn, &sc.initial)?;
        let snapshot: Vec<SnapshotGramType> = res.into_iter().map(Into::into).collect();

        assert_eq!(snapshot.len(), sc.initial.len());
        assert_matches_enum(&snapshot);
        insta::assert_debug_snapshot!(
            "[GramType::bulk_upsert] - after conflict(id) repair",
            snapshot
        );

        // 4) Force UPDATE via conflict(code): same code, wrong name (id can be different)
        {
            let tx = conn.transaction()?;
            DbQuery::execute(
                &tx,
                r#"
                INSERT INTO gram_type (id, code, name)
                VALUES (?1, ?2, ?3)
                "#,
                params![999i32, "verb_main", "WRONG_NAME_BY_CODE"],
            )?;
            tx.commit()?;
        }

        let res = GramTypeRepo::bulk_upsert(&mut conn, &sc.initial)?;
        let snapshot: Vec<SnapshotGramType> = res.into_iter().map(Into::into).collect();

        assert_eq!(snapshot.len(), sc.initial.len());
        assert_matches_enum(&snapshot);
        assert!(
            snapshot.iter().all(|r| r.gram.id() != 999),
            "the injected row (id=999) should not survive after upsert"
        );
        insta::assert_debug_snapshot!(
            "[GramType::bulk_upsert] - after conflict(code) repair",
            snapshot
        );

        Ok(())
    }
}
