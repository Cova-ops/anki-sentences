#[cfg(test)]
mod tests_gram_type_repo_bulk_upsert {
    use crate::test_utils::prelude::*; // setup_test_db(), snapshot/assert helpers, scenario, etc.
    use rusqlite::params;

    #[test]
    fn bulk_upsert_behaviour() -> Result<(), DbError> {
        let mut conn = setup_test_db()?;

        // 1) Empty
        let res = GramTypeRepo::bulk_upsert(&mut conn, &[])?;
        res.assert_eq_fields(&vec![]);
        insta::assert_debug_snapshot!("[GramType::bulk_upsert] - empty", res.snapshot());

        // 2) Normal insert
        let sc = scenario_gram_type();
        let res = GramTypeRepo::bulk_upsert(&mut conn, &sc.initial)?;
        res.assert_eq_fields(&sc.initial);
        insta::assert_debug_snapshot!("[GramType::bulk_upsert] - after insert", res.snapshot());

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
        res.assert_eq_fields(&sc.initial);
        insta::assert_debug_snapshot!(
            "[GramType::bulk_upsert] - after conflict(id) repair",
            res.snapshot()
        );

        // 4) Force UPDATE via conflict(code): same code, wrong name (id can be different)
        // Insert a row with the same code but wrong name. If code is UNIQUE in schema, this triggers conflict(code).
        // If code is not UNIQUE, this test won't make sense; you need UNIQUE(code).
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
        insta::assert_debug_snapshot!(
            "[GramType::bulk_upsert] - after conflict(code) repair",
            res.snapshot()
        );

        Ok(())
    }
}
