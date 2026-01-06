#[cfg(test)]
mod test_worte_gender_repo {
    use color_eyre::eyre::Result;
    use rusqlite::Connection;

    use crate::test_utils::prelude::*;

    mod bulk_upsert {

        use super::*;

        fn run_bulk_upsert_scenario<F>(insert_fn: F, sc: Scenario<NewWorteGenderSchema>)
        where
            F: Fn(&mut Connection, &[NewWorteGenderSchema]) -> Result<Vec<WorteGenderSchema>>,
        {
            let mut conn = setup_test_db().unwrap();

            let res_1 = insert_fn(&mut conn, &sc.initial).expect("La inserción no debe fallar");
            res_1.assert_eq_fields(&sc.initial);
            insta::assert_debug_snapshot!("after_insert", res_1.snapshot());

            let res_2 = insert_fn(&mut conn, &sc.update).expect("La inserción no debe fallar");
            res_2.assert_eq_fields(&sc.update);
            insta::assert_debug_snapshot!("after_update", res_2.snapshot());
        }

        #[test]
        fn test_bulk_insert_and_update() {
            let sc = scenario_worte_gender();
            run_bulk_upsert_scenario(|conn, data| WorteGenderRepo::bulk_insert(conn, data), sc);
        }
    }
}
