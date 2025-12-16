use crate::db::setup_test_db;

#[cfg(test)]
mod test_worte_gram_type_repo {

    use crate::db::{
        schemas::worte_gram_type::{NewWorteGramTypeSchema as New, WorteGramTypeSchema as Schema},
        worte_gram_type::WorteGramTypeRepo,
    };

    use super::*;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Snapshot {
        id_worte: i32,
        id_gram_type: i32,
        created_at: String,
        deleted_at: String,
    }

    fn placeholder_dates(data: Vec<Schema>) -> Vec<Snapshot> {
        data.into_iter()
            .map(|d| Snapshot {
                id_worte: d.id_worte,
                id_gram_type: d.id_gram_type,
                created_at: "<created_at>".into(),
                deleted_at: "<deleted_at>".into(),
            })
            .collect()
    }

    mod bulk_insert {
        use color_eyre::eyre::Result;
        use rusqlite::Connection;

        use crate::db::{schemas::worte::NewWorteSchema, seeders::init_data, worte::WorteRepo};

        use super::*;

        fn init_data_local(conn: &mut Connection) -> Result<()> {
            init_data(conn)?;
            WorteRepo::bulk_insert(
                conn,
                &[
                    NewWorteSchema {
                        gram_type: vec![],
                        gender_id: Some(1),
                        worte_de: "Hund".into(),
                        worte_es: "Perro".into(),
                        plural: Some("Hunde".into()),
                        niveau_id: 1,
                        example_de: "Beispiel".into(),
                        example_es: "Ejemplo".into(),
                        verb_aux: None,
                        trennbar: None,
                        reflexiv: None,
                    },
                    NewWorteSchema {
                        gram_type: vec![],
                        gender_id: None,
                        worte_de: "laufen".into(),
                        worte_es: "correr".into(),
                        plural: None,
                        niveau_id: 2,
                        example_de: "Beispiel".into(),
                        example_es: "Ejemplo".into(),
                        verb_aux: Some("sein".into()),
                        trennbar: Some(false),
                        reflexiv: Some(false),
                    },
                ],
            )?;
            Ok(())
        }

        fn run_bulk_insert_update_scenario<F>(c1: F)
        where
            F: FnOnce(&mut Connection) -> Result<Vec<Schema>>,
        {
            let mut conn = setup_test_db().unwrap();
            init_data_local(&mut conn).expect("Fallar al iniciar datos para testing");

            let res_1 = c1(&mut conn).expect("La inserción no debe fallar");

            assert_eq!(res_1.len(), 3);

            assert_eq!(res_1[0].id_worte, 1);
            assert_eq!(res_1[0].id_gram_type, 1);

            assert_eq!(res_1[1].id_worte, 1);
            assert_eq!(res_1[1].id_gram_type, 2);

            assert_eq!(res_1[2].id_worte, 2);
            assert_eq!(res_1[2].id_gram_type, 2);

            let res_1 = placeholder_dates(res_1);
            insta::assert_debug_snapshot!(res_1);
        }

        #[test]
        fn test_bulk_insert_and_update() {
            let data_1 = vec![
                New {
                    id_worte: 1,
                    id_gram_type: 1,
                },
                New {
                    id_worte: 1,
                    id_gram_type: 2,
                },
                New {
                    id_worte: 2,
                    id_gram_type: 2,
                },
            ];
            run_bulk_insert_update_scenario(|conn| WorteGramTypeRepo::bulk_insert(conn, &data_1));
        }

        #[test]
        fn test_bulk_insert_and_update_tx() {
            let data_1 = vec![
                New {
                    id_worte: 1,
                    id_gram_type: 1,
                },
                New {
                    id_worte: 1,
                    id_gram_type: 2,
                },
                New {
                    id_worte: 2,
                    id_gram_type: 2,
                },
            ];
            run_bulk_insert_update_scenario(|conn| {
                let tx = conn.transaction()?;
                let out = WorteGramTypeRepo::bulk_insert_tx(&tx, &data_1)?;
                tx.commit()?;
                Ok(out)
            });
        }
    }
}
