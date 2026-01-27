#[cfg(test)]
mod test_worte_repo {

    use color_eyre::eyre::Result;
    use rusqlite::Connection;

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

        Ok(())
    }

    mod insert {
        use super::*;

        #[test]
        fn test_bulk_insert() -> Result<()> {
            let mut conn = setup_test_db().unwrap();
            data_minimun(&mut conn)?;

            let sc = scenario_worte();

            let res = WorteRepo::bulk_insert(&mut conn, &[])?;
            res.assert_eq_fields(&vec![]);
            insta::assert_debug_snapshot!("[Worte::bulk_insert] - empty", res.snapshot());

            let res = WorteRepo::bulk_insert(&mut conn, &sc.initial)?;
            res.assert_eq_fields(&sc.initial);
            insta::assert_debug_snapshot!("[Worte::bulk_insert] - insert", res.snapshot());

            Ok(())
        }
    }

    mod fetch {
        use super::*;

        fn init_data_local(conn: &mut Connection, sc: &Scenario<NewWorteSchema>) -> Result<()> {
            data_minimun(conn)?;

            WorteRepo::bulk_insert(conn, &sc.initial)?;

            Ok(())
        }

        #[test]
        fn test_fetch_by_id() -> Result<()> {
            let mut conn = setup_test_db()?;

            let sc = scenario_worte();
            init_data_local(&mut conn, &sc)?;

            let res = WorteRepo::fetch_by_id(&conn, &[])?;
            res.assert_eq_fields(&vec![]);
            insta::assert_debug_snapshot!("[Worte::fetch_by_id] - empty", res.snapshot());

            let res = WorteRepo::fetch_by_id(&conn, &[1, 2, 3])?;

            // We take only the three first of the array
            let data_compare: Vec<NewWorteSchema> = sc.initial.into_iter().take(3).collect();
            res.assert_eq_fields(&data_compare);
            insta::assert_debug_snapshot!("[Worte::fetch_by_id] - fetch", res.snapshot());

            Ok(())
        }

        #[test]
        fn test_fetch_all_ids() -> Result<()> {
            let mut conn = setup_test_db()?;

            let sc = scenario_worte();
            init_data_local(&mut conn, &sc)?;

            let limit = 2;
            let mut last_id = 0;

            let mut res: Vec<i32> = vec![];
            loop {
                let mut a = WorteRepo::fetch_all_ids(&conn, limit, last_id)?;
                if a.is_empty() {
                    break;
                }

                last_id = a.last().unwrap().clone();
                res.append(&mut a);
            }

            let len_compare: usize = sc.initial.len();
            assert_eq!(res.len(), len_compare);

            let vec_compare: Vec<i32> = (1..=len_compare as i32).collect();
            assert_eq!(res, vec_compare);

            insta::assert_debug_snapshot!("[Worte::fetch_all_ids] - fetch", res);

            Ok(())
        }

        #[test]
        fn test_fetch_by_wort() -> Result<()> {
            let mut conn = setup_test_db()?;

            let sc = scenario_worte();
            init_data_local(&mut conn, &sc)?;

            let res = WorteRepo::fetch_by_wort(&conn, &[])?;
            res.assert_eq_fields(&vec![]);
            insta::assert_debug_snapshot!("[Wort::fetch_by_wort] - empty", res.snapshot());

            let mut worte_fetched: Vec<(String, String)> = sc
                .initial
                .iter()
                .take(5)
                .map(|w| (w.worte_es.clone(), w.worte_de.clone()))
                .collect();

            let last = worte_fetched.last_mut();
            *last.unwrap() = ("Test test tes".to_owned(), "1234567890".to_owned());

            let res = WorteRepo::fetch_by_wort(&conn, &worte_fetched)?;

            let data_compare: Vec<NewWorteSchema> = sc.initial.into_iter().take(4).collect();
            res.assert_eq_fields(&data_compare);

            insta::assert_debug_snapshot!("[Wort::fetch_by_wort] - fetch", res.snapshot());

            Ok(())
        }
    }

    mod update {
        use super::*;

        fn init_data_local(conn: &mut Connection, sc: &Scenario<NewWorteSchema>) -> Result<()> {
            data_minimun(conn)?;

            WorteRepo::bulk_insert(conn, &sc.initial)?;

            Ok(())
        }

        #[test]
        fn test_bulk_update() -> Result<()> {
            let mut conn = setup_test_db().unwrap();

            let sc = scenario_worte();
            init_data_local(&mut conn, &sc)?;

            let res = WorteRepo::bulk_update(&mut conn, &[])?;
            res.assert_eq_fields(&vec![]);
            insta::assert_debug_snapshot!("[Wort::bulk_update] - empty", res.snapshot());

            let data_update: Vec<(i32, NewWorteSchema)> = sc
                .update
                .iter()
                .enumerate()
                .map(|(i, w)| ((i + 1) as i32, w.clone()))
                .collect();

            let res = WorteRepo::bulk_update(&mut conn, &data_update)?;
            res.assert_eq_fields(&sc.update);
            insta::assert_debug_snapshot!("[Wort::bulk_update] - update", res.snapshot());

            Ok(())
        }
    }
}
