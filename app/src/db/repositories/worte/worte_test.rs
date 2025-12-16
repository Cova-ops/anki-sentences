use crate::db::setup_test_db;

#[cfg(test)]
mod test_worte_repo {

    use crate::db::{
        schemas::worte::{NewWorteSchema as New, WorteSchema as Schema},
        worte::WorteRepo,
    };

    use super::*;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Snapshot {
        id: i32,
        gender_id: Option<GenderWorteSnapshot>,
        worte_de: String,
        worte_es: String,
        plural: Option<String>,
        niveau_id: NiveauSnapshot,
        example_de: String,
        example_es: String,

        // nur verben
        verb_aux: Option<String>,
        trennbar: Option<bool>,
        reflexiv: Option<bool>,

        created_at: String,
        deleted_at: String,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct NiveauSnapshot {
        id: i32,
        niveau: String,
        created_at: String,
        deleted_at: String,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct GenderWorteSnapshot {
        id: i32,
        gender: String,
        artikel: String,
        created_at: String,
        deleted_at: String,
    }

    fn placeholder_dates(data: Vec<Schema>) -> Vec<Snapshot> {
        data.into_iter()
            .map(|d| Snapshot {
                id: d.id,
                gender_id: if let Some(gender) = d.gender_id {
                    Some(GenderWorteSnapshot {
                        id: gender.id,
                        gender: gender.gender,
                        artikel: gender.artikel,
                        created_at: "<created_at>".into(),
                        deleted_at: "<deleted_at>".into(),
                    })
                } else {
                    None
                },
                worte_de: d.worte_de,
                worte_es: d.worte_es,
                plural: d.plural,
                niveau_id: NiveauSnapshot {
                    id: d.niveau_id.id,
                    niveau: d.niveau_id.niveau,
                    created_at: "<created_at>".into(),
                    deleted_at: "<deleted_at>".into(),
                },
                example_de: d.example_de,
                example_es: d.example_es,

                // nur verben
                verb_aux: d.verb_aux,
                trennbar: d.trennbar,
                reflexiv: d.reflexiv,

                created_at: "<created_at>".into(),
                deleted_at: "<deleted_at>".into(),
            })
            .collect()
    }

    mod bulk_insert {
        use color_eyre::eyre::Result;
        use rusqlite::Connection;

        use crate::db::seeders::init_data;

        use super::*;

        fn init_data_local(conn: &mut Connection) -> Result<()> {
            init_data(conn)?;
            Ok(())
        }

        fn run_bulk_insert_update_scenario<F>(c1: F)
        where
            F: FnOnce(&mut Connection) -> Result<Vec<Schema>>,
        {
            let mut conn = setup_test_db().unwrap();
            init_data_local(&mut conn).expect("Error al iniciar datos dummy");

            let res_1 = c1(&mut conn).expect("La inserción no debe fallar");

            assert_eq!(res_1.len(), 2);

            assert_eq!(res_1[0].id, 1);
            assert_eq!(res_1[0].gram_type_id[0].id, 1);
            assert_eq!(res_1[0].gender_id.as_ref().unwrap().id, 1);
            assert_eq!(res_1[0].worte_de, "Hund");
            assert_eq!(res_1[0].worte_es, "Perro");
            assert_eq!(res_1[0].plural, Some("Hunde".into()));
            assert_eq!(res_1[0].niveau_id.id, 1);
            assert_eq!(res_1[0].example_de, "Beispiel");
            assert_eq!(res_1[0].example_es, "Ejemplo");
            assert_eq!(res_1[0].verb_aux, None);
            assert_eq!(res_1[0].trennbar, None);
            assert_eq!(res_1[0].reflexiv, None);

            assert_eq!(res_1[1].id, 2);
            assert_eq!(res_1[1].gram_type_id[0].id, 2);
            assert_eq!(res_1[1].gram_type_id[1].id, 3);
            assert_eq!(res_1[1].gender_id, None);
            assert_eq!(res_1[1].worte_de, "laufen");
            assert_eq!(res_1[1].worte_es, "correr");
            assert_eq!(res_1[1].plural, None);
            assert_eq!(res_1[1].niveau_id.id, 2);
            assert_eq!(res_1[1].example_de, "Beispiel");
            assert_eq!(res_1[1].example_es, "Ejemplo");
            assert_eq!(res_1[1].verb_aux, Some("sein".into()));
            assert_eq!(res_1[1].trennbar, Some(false));
            assert_eq!(res_1[1].reflexiv, Some(false));

            let res_1 = placeholder_dates(res_1);
            insta::assert_debug_snapshot!(res_1);
        }

        #[test]
        fn test_bulk_insert() {
            let data_1 = vec![
                New {
                    gram_type: vec![1],
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
                New {
                    gram_type: vec![2, 3],
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
            ];
            run_bulk_insert_update_scenario(|conn| WorteRepo::bulk_insert(conn, &data_1));
        }

        #[test]
        fn test_bulk_insert_and_update_tx() {
            let data_1 = vec![
                New {
                    gram_type: vec![1],
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
                New {
                    gram_type: vec![2, 3],
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
            ];

            run_bulk_insert_update_scenario(|conn| {
                let tx = conn.transaction()?;
                let out = WorteRepo::bulk_insert_tx(&tx, &data_1)?;
                tx.commit()?;
                Ok(out)
            });
        }
    }

    mod fetch {

        use super::*;
        use color_eyre::eyre::Result;
        use rusqlite::Connection;

        use crate::{
            db::{
                schemas::worte_review::NewWorteReviewSchema, seeders::init_data, setup_test_db,
                worte_review::WorteReviewRepo,
            },
            helpers::time::fixed_date,
        };

        fn init_data_local(conn: &mut Connection) -> Result<()> {
            init_data(conn)?;
            let data = [
                New {
                    gram_type: vec![1],
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
                New {
                    gram_type: vec![2, 3],
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
            ];
            WorteRepo::bulk_insert(conn, &data)?;
            Ok(())
        }

        #[test]
        fn test_fetch_by_id() {
            let mut conn = setup_test_db().unwrap();
            init_data_local(&mut conn).expect("Error al iniciar datos dummy");

            let res_1 = WorteRepo::fetch_by_id(&conn, &[1, 2]).expect("Error al hacer el fetch");

            assert_eq!(res_1.len(), 2);
            assert_eq!(res_1[0].worte_de, "Hund");
            assert_eq!(res_1[0].gram_type_id.len(), 1);
            assert_eq!(res_1[1].worte_de, "laufen");

            let res_1 = placeholder_dates(res_1);
            insta::assert_debug_snapshot!(res_1);
        }

        #[test]
        fn test_fetch_id_neue_worte() {
            let mut conn = setup_test_db().unwrap();
            init_data_local(&mut conn).expect("Error al iniciar datos dummy");

            let res_1 = WorteRepo::fetch_id_neue_worte(&conn).expect("Error al hacer el fetch");

            assert_eq!(res_1.len(), 2);
            assert_eq!(res_1[0], 1);
            assert_eq!(res_1[1], 2);

            insta::assert_debug_snapshot!(res_1);

            WorteReviewRepo::bulk_insert(
                &mut conn,
                &[NewWorteReviewSchema {
                    wort_id: 1,
                    repetitions: 1,
                    ease_factor: 2.0,
                    interval: 1,
                    last_review: "2025-01-10 12:00:00".into(),
                    next_review: "2025-01-10 12:00:00".into(),
                }],
            )
            .expect("Error al hacer el insert de worte review");

            let res_2 = WorteRepo::fetch_id_neue_worte(&conn).expect("Error al hacer el fetch");

            assert_eq!(res_2.len(), 1);
            assert_eq!(res_2[0], 2);

            insta::assert_debug_snapshot!(res_2);
        }
    }
}
