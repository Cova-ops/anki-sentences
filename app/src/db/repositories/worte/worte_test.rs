#[cfg(test)]
mod test_worte_repo {

    use color_eyre::eyre::Result;
    use rusqlite::Connection;

    use crate::{
        db::{
            gram_type::GramTypeRepo,
            niveau_liste::NiveauListeRepo,
            schemas::worte::{NewWorteSchema as New, WorteSchema as Schema},
            setup_test_db,
            worte::WorteRepo,
            worte_gender::WorteGenderRepo,
        },
        test_utils::{
            scenarios::{
                Scenario, gram_type::scenario_gram_type_schema,
                niveau_liste::scenario_niveau_liste_schema, worte::scenario_worte_schema,
                worte_gender::scenario_worte_gender_schema,
            },
            traits::{AssertEqFields, SnapshotFields},
        },
    };

    fn data_minimun(conn: &mut Connection) -> Result<()> {
        let sc = scenario_worte_gender_schema();
        WorteGenderRepo::bulk_insert(conn, &sc.initial)?;

        let sc = scenario_gram_type_schema();
        GramTypeRepo::bulk_insert(conn, &sc.initial)?;

        let sc = scenario_niveau_liste_schema();
        NiveauListeRepo::bulk_insert(conn, &sc.initial)?;

        Ok(())
    }

    mod bulk_insert {

        use super::*;

        fn run_bulk_insert_scenario<F>(insert_fn: F, sc: Scenario<New>)
        where
            F: Fn(&mut Connection, &[New]) -> Result<Vec<Schema>>,
        {
            let mut conn = setup_test_db().unwrap();
            data_minimun(&mut conn).expect("Error al iniciar datos dummy");

            let res_1 = insert_fn(&mut conn, &sc.initial).expect("La inserción no debe fallar");

            res_1.assert_eq_fields(&sc.initial);

            insta::assert_debug_snapshot!("after_insert", res_1.snapshot());
        }

        #[test]
        fn test_bulk_insert() {
            let sc = scenario_worte_schema();
            run_bulk_insert_scenario(|conn, data| WorteRepo::bulk_insert(conn, data), sc);
        }
    }

    mod fetch {

        use super::*;

        fn init_data_local(conn: &mut Connection, sc: &Scenario<New>) -> Result<()> {
            data_minimun(conn)?;

            WorteRepo::bulk_insert(conn, &sc.initial)?;

            Ok(())
        }

        #[test]
        fn test_fetch_by_id() {
            let mut conn = setup_test_db().unwrap();

            let sc = scenario_worte_schema();
            init_data_local(&mut conn, &sc).expect("Error al iniciar datos dummy");

            let res_1 = WorteRepo::fetch_by_id(&conn, &[1, 2]).expect("Error al hacer el fetch");

            // We take only the two first of the array
            let data_compare: Vec<New> = sc.initial.into_iter().take(2).collect();
            res_1.assert_eq_fields(&data_compare);
            insta::assert_debug_snapshot!(res_1.snapshot());
        }
    }
}
