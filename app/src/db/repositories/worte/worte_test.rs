#[cfg(test)]
mod test_worte_repo {

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

        Ok(())
    }

    mod insert {
        use super::*;

        fn run_bulk_insert_scenario<F>(insert_fn: F, sc: Scenario<NewWorteSchema>)
        where
            F: Fn(&mut Connection, &[NewWorteSchema]) -> Result<Vec<WorteSchema>>,
        {
            let mut conn = setup_test_db().unwrap();
            data_minimun(&mut conn).expect("Error al iniciar datos dummy");

            let res = insert_fn(&mut conn, &sc.initial).expect("La inserción no debe fallar");

            res.assert_eq_fields(&sc.initial);

            insta::assert_debug_snapshot!("after_insert", res.snapshot());
        }

        #[test]
        fn test_bulk_insert() {
            let sc = scenario_worte();
            run_bulk_insert_scenario(|conn, data| WorteRepo::bulk_insert(conn, data), sc);
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
        fn test_fetch_by_id() {
            let mut conn = setup_test_db().unwrap();

            let sc = scenario_worte();
            init_data_local(&mut conn, &sc).expect("Error al iniciar datos dummy");

            let res = WorteRepo::fetch_by_id(&conn, &[1, 2, 3]).expect("Error al hacer el fetch");

            // We take only the two first of the array
            let data_compare: Vec<NewWorteSchema> = sc.initial.into_iter().take(3).collect();
            res.assert_eq_fields(&data_compare);
            insta::assert_debug_snapshot!("fetch_by_id", res.snapshot());
        }

        #[test]
        fn test_fetch_all_ids() {
            let mut conn = setup_test_db().unwrap();

            let sc = scenario_worte();
            init_data_local(&mut conn, &sc).expect("Error al iniciar datos dummy");

            let limit = 2;
            let mut last_id = 0;

            let mut res: Vec<i32> = vec![];
            loop {
                let mut a = WorteRepo::fetch_all_ids(&conn, limit, last_id)
                    .expect("Error al hacer el fetch");

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

            insta::assert_debug_snapshot!("fetch_all_ids", res);
        }

        #[test]
        fn test_fetch_by_wort() {
            let mut conn = setup_test_db().unwrap();

            let sc = scenario_worte();
            init_data_local(&mut conn, &sc).expect("Error al iniciar datos dummy");

            let mut worte_fetched: Vec<(String, String)> = sc
                .initial
                .iter()
                .take(5)
                .map(|w| (w.worte_es.clone(), w.worte_de.clone()))
                .collect();

            let last = worte_fetched.last_mut();
            *last.unwrap() = ("Test test tes".to_owned(), "1234567890".to_owned());

            let res =
                WorteRepo::fetch_by_wort(&conn, &worte_fetched).expect("Error al hacer el fetch");

            let data_compare: Vec<NewWorteSchema> = sc.initial.into_iter().take(4).collect();
            res.assert_eq_fields(&data_compare);

            insta::assert_debug_snapshot!("fetch_by_wort", res.snapshot());
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

            let data_update: Vec<(i32, NewWorteSchema)> = sc
                .update
                .iter()
                .enumerate()
                .map(|(i, w)| ((i + 1) as i32, w.clone()))
                .collect();

            let res = WorteRepo::bulk_update(&mut conn, &data_update)?;

            res.assert_eq_fields(&sc.update);

            insta::assert_debug_snapshot!("bulk_update", res.snapshot());

            Ok(())
        }
    }
}
