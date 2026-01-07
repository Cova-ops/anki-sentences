#[cfg(test)]
mod test_worte_gram_type_repo {

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

        let mut sc = scenario_worte();

        // We need to change this, in other case the test will failed, cause there are information
        // added that it is not in te ScenarioWorteGramType
        sc.initial = sc
            .initial
            .into_iter()
            .map(|w| NewWorteSchema {
                gram_type: vec![],
                ..w.clone()
            })
            .collect();
        WorteRepo::bulk_insert(conn, &sc.initial)?;

        Ok(())
    }

    mod insert {
        use super::*;

        #[test]
        fn test_bulk_insert() -> Result<()> {
            let mut conn = setup_test_db().unwrap();
            data_minimun(&mut conn)?;

            let sc = scenario_worte_gram_type();

            let res = WorteGramTypeRepo::bulk_insert(&mut conn, &[])?;
            res.assert_eq_fields(&vec![]);
            insta::assert_debug_snapshot!("empty_insert", res.snapshot());

            let res = WorteGramTypeRepo::bulk_insert(&mut conn, &sc.initial)?;
            res.assert_eq_fields(&sc.initial);
            insta::assert_debug_snapshot!("bulk_insert", res.snapshot());

            Ok(())
        }
    }

    mod fetch {
        use super::*;

        fn insert_fields(conn: &mut Connection) -> Result<()> {
            let sc = scenario_worte_gram_type();
            WorteGramTypeRepo::bulk_insert(conn, &sc.initial)?;

            Ok(())
        }

        #[test]
        fn test_fetch_by_wort_id() -> Result<()> {
            let mut conn = setup_test_db().unwrap();
            data_minimun(&mut conn)?;
            insert_fields(&mut conn)?;

            let sc = scenario_worte_gram_type();

            let res = WorteGramTypeRepo::fetch_by_wort_id(&conn, &[])?;
            res.assert_eq_fields(&vec![]);
            insta::assert_debug_snapshot!("empty_fetch", res.snapshot());

            let id_fetch = vec![1];
            let res = WorteGramTypeRepo::fetch_by_wort_id(&conn, &id_fetch)?;
            let data_compared: Vec<NewWorteGramTypeSchema> = sc
                .initial
                .iter()
                .filter(|w| id_fetch.contains(&w.id_worte))
                .cloned()
                .collect();

            res.assert_eq_fields(&data_compared);
            insta::assert_debug_snapshot!(
                "[WorteGramType::fetch_by_wort_id] - some_fetch",
                res.snapshot()
            );

            Ok(())
        }

        #[test]
        fn test_fetch_all_worte_id() -> Result<()> {
            let mut conn = setup_test_db().unwrap();
            data_minimun(&mut conn)?;
            insert_fields(&mut conn)?;

            let sc = scenario_worte_gram_type();

            let limit: usize = 1;
            let mut last_id: i32 = 0;

            let mut vec_res: Vec<i32> = vec![];
            loop {
                let mut res = WorteGramTypeRepo::fetch_all_worte_id(&conn, limit, last_id)?;

                if res.is_empty() {
                    break;
                }

                last_id = res.last().unwrap().clone();
                vec_res.append(&mut res);
            }

            let mut data_compared: Vec<i32> = sc.initial.into_iter().map(|w| w.id_worte).collect();
            data_compared.sort_unstable();
            data_compared.dedup();

            assert_eq!(vec_res.len(), data_compared.len());
            for (r, d) in vec_res.iter().zip(data_compared.iter()) {
                assert_eq!(r, d);
            }
            insta::assert_debug_snapshot!(
                "[WorteGramType::fetch_all_worte_id] - some_fetch",
                vec_res
            );

            Ok(())
        }
    }
}
