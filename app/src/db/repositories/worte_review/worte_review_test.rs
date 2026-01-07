#[cfg(test)]
mod test_worte_review_repo {

    use color_eyre::eyre::Result;
    use rusqlite::Connection;
    use std::collections::HashSet;

    use crate::test_utils::prelude::*;

    fn data_minimun(conn: &mut Connection) -> Result<()> {
        let sc = scenario_worte_gender();
        let data = WorteGenderRepo::bulk_upsert(conn, &sc.initial)?;
        WorteGenderSchema::init_data(&data);

        let sc = scenario_gram_type();
        let data = GramTypeRepo::bulk_upsert(conn, &sc.initial)?;
        GramTypeSchema::init_data(&data);

        let sc = scenario_niveau_liste();
        let data = NiveauListeRepo::bulk_upsert(conn, &sc.initial)?;
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

            let res = WorteReviewRepo::bulk_upsert(&mut conn, &[])?;
            res.assert_eq_fields(&vec![]);
            insta::assert_debug_snapshot!("[WorteReview::bulk_upsert] - empty", res.snapshot());

            let res = WorteReviewRepo::bulk_upsert(&mut conn, &sc.initial)?;
            res.assert_eq_fields(&sc.initial);
            insta::assert_debug_snapshot!(
                "[WorteReview::bulk_upsert] - after insert",
                res.snapshot()
            );

            let res = WorteReviewRepo::bulk_upsert(&mut conn, &sc.update)?;
            res.assert_eq_fields(&sc.update);
            insta::assert_debug_snapshot!(
                "[WorteReview::bulk_upsert] - after update",
                res.snapshot()
            );

            Ok(())
        }
    }

    mod fetch {
        use super::*;

        #[test]
        fn test_fetch_by_wort_id() -> Result<()> {
            let mut conn = setup_test_db()?;
            data_minimun(&mut conn)?;

            let sc = scenario_worte_review();
            WorteReviewRepo::bulk_upsert(&mut conn, &sc.initial)?;

            let res = WorteReviewRepo::fetch_by_wort_id(&conn, &[])?;
            res.assert_eq_fields(&vec![]);
            insta::assert_debug_snapshot!(
                "[WorteReview::fetch_by_wort_id] - empty",
                res.snapshot()
            );

            let mut data = sc
                .initial
                .iter()
                .map(|w| w.wort_id.clone())
                .take(2)
                .collect::<Vec<i32>>();

            data.push(-32); // This ID should never exist
            let res = WorteReviewRepo::fetch_by_wort_id(&conn, &data)?;

            let data_compared: Vec<NewWorteReviewSchema> = sc.initial.into_iter().take(2).collect();
            res.assert_eq_fields(&data_compared);
            insta::assert_debug_snapshot!(
                "[WorteReview::fetch_by_wort_id] - fetch",
                res.snapshot()
            );

            Ok(())
        }

        #[test]
        fn test_fetch_new_wort_id_4_review() -> Result<()> {
            fn fetch_new_ids_both(conn: &Connection) -> Result<Vec<i32>> {
                let mut res =
                    WorteReviewRepo::fetch_new_wort_id_4_review(conn, ReviewDirection::ES2DE)?;
                let mut res2 =
                    WorteReviewRepo::fetch_new_wort_id_4_review(conn, ReviewDirection::DE2ES)?;

                res.append(&mut res2);
                res.sort_unstable();
                Ok(res)
            }

            let mut conn = setup_test_db()?;
            data_minimun(&mut conn)?;

            let sc_wort = scenario_worte();

            // If there are no reviews, ALL IDs for both directions should be returned,
            // but since the query returns IDs (not pairs), you'll end up with duplicate IDs. let n = sc_wort.initial.len();
            let n = sc_wort.initial.len();
            let mut data_compared = Vec::with_capacity(n * 2);
            for id in 1..=n as i32 {
                data_compared.push(id); // ES2DE
                data_compared.push(id); // DE2ES
            }
            data_compared.sort_unstable();

            let res = fetch_new_ids_both(&conn)?;
            assert_eq!(res, data_compared);
            insta::assert_debug_snapshot!(
                "[WorteReview::fetch_new_wort_id_4_review] - initial",
                res
            );

            let sc = scenario_worte_review();
            WorteReviewRepo::bulk_upsert(&mut conn, &sc.initial)?;

            let sc_ids: HashSet<(i32, ReviewDirection)> = sc
                .initial
                .iter()
                .map(|w| -> Result<(i32, ReviewDirection)> {
                    Ok((w.wort_id, ReviewDirection::try_from(w.direction.as_str())?))
                })
                .collect::<Result<HashSet<_>>>()?;

            let mut data_compared = vec![];
            for id in 1..=n as i32 {
                if !sc_ids.contains(&(id, ReviewDirection::ES2DE)) {
                    data_compared.push(id);
                }
                if !sc_ids.contains(&(id, ReviewDirection::DE2ES)) {
                    data_compared.push(id);
                }
            }
            data_compared.sort_unstable();

            let res = fetch_new_ids_both(&conn)?;
            assert_eq!(res, data_compared);
            insta::assert_debug_snapshot!(
                "[WorteReview::fetch_new_wort_id_4_review] - after insert",
                res
            );

            Ok(())
        }
    }
}
