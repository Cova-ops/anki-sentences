use crate::db::setup_test_db;

#[cfg(test)]
mod test_worte_review_repo {

    use crate::db::{
        schemas::worte_review::{NewWorteReviewSchema as New, WorteReviewSchema as Schema},
        worte_review::WorteReviewRepo,
    };

    use super::*;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Snapshot {
        id: i32,
        wort_id: i32,
        interval: u32,
        ease_factor: f32,
        repetitions: u32,
        last_review: String,
        next_review: String,

        created_at: String,
        deleted_at: String,
    }

    fn placeholder_dates(data: Vec<Schema>) -> Vec<Snapshot> {
        data.into_iter()
            .map(|d| Snapshot {
                id: d.id,

                wort_id: d.wort_id,
                interval: d.interval,
                ease_factor: d.ease_factor,
                repetitions: d.repetitions,
                last_review: d.last_review.to_string(),
                next_review: d.next_review.to_string(),

                created_at: "<created_at>".into(),
                deleted_at: "<deleted_at>".into(),
            })
            .collect()
    }

    mod bulk_insert {

        use color_eyre::eyre::Result;
        use rusqlite::Connection;

        use crate::{
            db::{schemas::worte::NewWorteSchema, seeders::init_data, worte::WorteRepo},
            helpers::time::fixed_date,
        };

        use super::*;

        fn init_data_local(conn: &mut Connection) -> Result<()> {
            init_data(conn)?;
            let data = vec![
                NewWorteSchema {
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
                NewWorteSchema {
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

        fn run_bulk_insert_update_scenario<F1, F2>(c1: F1, c2: F2)
        where
            F1: FnOnce(&mut Connection) -> Result<Vec<Schema>>,
            F2: FnOnce(&mut Connection) -> Result<Vec<Schema>>,
        {
            let mut conn = setup_test_db().unwrap();
            init_data_local(&mut conn).expect("Error al iniciar datos dummy");

            let res_1 = c1(&mut conn).expect("La inserción no debe fallar");

            assert_eq!(res_1.len(), 1);

            assert_eq!(res_1[0].wort_id, 1);
            assert_eq!(res_1[0].interval, 1);
            assert_eq!(res_1[0].ease_factor, 2.5);
            assert_eq!(res_1[0].repetitions, 999);
            assert_eq!(res_1[0].last_review, fixed_date(2025, 1, 10, 12, 00, 00));
            assert_eq!(res_1[0].next_review, fixed_date(2025, 1, 20, 12, 00, 00));

            let res_1 = placeholder_dates(res_1);
            insta::assert_debug_snapshot!(res_1);

            let res_2 = c2(&mut conn).expect("La inserción no debe fallar");

            assert_eq!(res_2.len(), 1);

            assert_eq!(res_2[0].wort_id, 1);
            assert_eq!(res_2[0].interval, 10);
            assert_eq!(res_2[0].ease_factor, 1.3);
            assert_eq!(res_2[0].repetitions, 1);
            assert_eq!(res_2[0].last_review, fixed_date(2025, 12, 10, 12, 00, 00));
            assert_eq!(res_2[0].next_review, fixed_date(2025, 12, 20, 12, 00, 00));

            let res_2 = placeholder_dates(res_2);
            insta::assert_debug_snapshot!(res_2);
        }

        #[test]
        fn test_bulk_insert() {
            let data_1 = vec![New {
                wort_id: 1,
                interval: 1,
                ease_factor: 2.5,
                repetitions: 999,
                last_review: "2025-01-10 12:00:00".into(),
                next_review: "2025-01-20 12:00:00".into(),
            }];
            let data_2 = vec![New {
                wort_id: 1,
                interval: 10,
                ease_factor: 1.3,
                repetitions: 1,
                last_review: "2025-12-10 12:00:00".into(),
                next_review: "2025-12-20 12:00:00".into(),
            }];
            run_bulk_insert_update_scenario(
                |conn| WorteReviewRepo::bulk_insert(conn, &data_1),
                |conn| WorteReviewRepo::bulk_insert(conn, &data_2),
            );
        }

        #[test]
        fn test_bulk_insert_and_update_tx() {
            let data_1 = vec![New {
                wort_id: 1,
                interval: 1,
                ease_factor: 2.5,
                repetitions: 999,
                last_review: "2025-01-10 12:00:00".into(),
                next_review: "2025-01-20 12:00:00".into(),
            }];
            let data_2 = vec![New {
                wort_id: 1,
                interval: 10,
                ease_factor: 1.3,
                repetitions: 1,
                last_review: "2025-12-10 12:00:00".into(),
                next_review: "2025-12-20 12:00:00".into(),
            }];

            run_bulk_insert_update_scenario(
                |conn| {
                    let tx = conn.transaction()?;
                    let out = WorteReviewRepo::bulk_insert_tx(&tx, &data_1)?;
                    tx.commit()?;
                    Ok(out)
                },
                |conn| {
                    let tx = conn.transaction()?;
                    let out = WorteReviewRepo::bulk_insert_tx(&tx, &data_2)?;
                    tx.commit()?;
                    Ok(out)
                },
            );
        }
    }
}
