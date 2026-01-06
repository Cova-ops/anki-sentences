#[cfg(test)]
mod test_worte_review_repo {

    use color_eyre::eyre::Result;
    use rusqlite::Connection;

    use crate::test_utils::prelude::*;

    fn data_minimun(conn: &mut Connection) -> Result<()> {
        let sc = scenario_worte_gender();
        let data = WorteGenderRepo::bulk_insert(conn, &sc.initial)?;
        WorteGenderSchema::init_data(&data);

        let sc = scenario_gram_type();
        let data = GramTypeRepo::bulk_insert(conn, &sc.initial)?;
        GramTypeSchema::init_data(&data);

        let sc = scenario_niveau_liste();
        let data = NiveauListeRepo::bulk_insert(conn, &sc.initial)?;
        NiveauListeSchema::init_data(&data);

        let sc = scenario_worte();
        WorteRepo::bulk_insert(conn, &sc.initial)?;

        Ok(())
    }

    mod bulk_insert {
        use super::*;

        #[test]
        fn test_bulk_upsert() -> Result<()> {
            let mut conn = setup_test_db().unwrap();
            data_minimun(&mut conn)?;

            let sc = scenario_worte_review();

            let res_1 = WorteReviewRepo::bulk_insert(&mut conn, &sc.initial)?;
            res_1.assert_eq_fields(&sc.initial);
            insta::assert_debug_snapshot!("after_insert", res_1.snapshot());

            let res_2 = WorteReviewRepo::bulk_insert(&mut conn, &sc.update)?;
            res_2.assert_eq_fields(&sc.update);
            insta::assert_debug_snapshot!("after_update", res_2.snapshot());

            Ok(())
        }
    }

    mod fetch {

        use super::*;

        #[test]
        fn test_fetch_by_wort_id() -> Result<()> {
            let mut conn = setup_test_db().unwrap();
            data_minimun(&mut conn)?;

            let sc = scenario_worte_review();
            WorteReviewRepo::bulk_insert(&mut conn, &sc.initial)?;

            let res = WorteReviewRepo::fetch_by_wort_id(&conn, &[])?;
            res.assert_eq_fields(&vec![]);
            insta::assert_debug_snapshot!("fetch_empty", res);

            let mut data = sc
                .initial
                .iter()
                .map(|w| w.wort_id.clone())
                .take(2)
                .collect::<Vec<i32>>();

            data.push(-32); // This ID should never exist
            let res = WorteReviewRepo::fetch_by_wort_id(&conn, &data)
                .expect("La consulta no debe fallar");

            let data_compared: Vec<NewWorteReviewSchema> = sc.initial.into_iter().take(2).collect();
            res.assert_eq_fields(&data_compared);
            insta::assert_debug_snapshot!("fetch_by_wort_id", res.snapshot());

            Ok(())
        }
    }
}
