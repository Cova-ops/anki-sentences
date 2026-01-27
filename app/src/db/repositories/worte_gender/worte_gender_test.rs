#[cfg(test)]
mod test_worte_gender_repo {
    use color_eyre::eyre::Result;

    use crate::test_utils::prelude::*;

    mod bulk_upsert {
        use super::*;

        #[test]
        fn test_bulk_upsert() -> Result<()> {
            let mut conn = setup_test_db()?;

            let res = WorteGenderRepo::bulk_upsert(&mut conn, &[])?;
            res.assert_eq_fields(&vec![]);
            insta::assert_debug_snapshot!("[WorteGender::bulk_upsert] - empty", res.snapshot());

            let sc = scenario_worte_gender();

            let res = WorteGenderRepo::bulk_upsert(&mut conn, &sc.initial)?;
            res.assert_eq_fields(&sc.initial);
            insta::assert_debug_snapshot!(
                "[WorteGender::bulk_upsert] - after insert",
                res.snapshot()
            );

            let res = WorteGenderRepo::bulk_upsert(&mut conn, &sc.update)?;
            res.assert_eq_fields(&sc.update);
            insta::assert_debug_snapshot!(
                "[WorteGender::bulk_upsert] - after update",
                res.snapshot()
            );

            Ok(())
        }
    }
}
