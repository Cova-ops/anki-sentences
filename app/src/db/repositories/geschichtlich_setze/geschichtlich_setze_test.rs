use crate::db::setup_test_db;

#[cfg(test)]
mod test_geschichtlich_setze_repo {

    use crate::db::{
        geschichtlich_setze::GeschichtlichSetzeRepo,
        schemas::geschichtlich_setze::{
            GeschichtlichSetzeSchema as Schema, NewGeschichtlichSetzeSchema as New,
        },
    };

    use super::*;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Snapshot {
        id: i32,
        setze_id: i32,
        result: bool,
        created_at: String,
        deleted_at: String,
    }

    fn placeholder_dates(data: Vec<Schema>) -> Vec<Snapshot> {
        data.into_iter()
            .map(|d| Snapshot {
                id: d.id,
                setze_id: d.setze_id,
                result: d.result,
                created_at: "<created_at>".into(),
                deleted_at: "<deleted_at>".into(),
            })
            .collect()
    }

    mod bulk_insert {
        use crate::db::{schemas::setze::NewSetzeSchema, seeders::init_data, setze::SetzeRepo};

        use super::*;
        use color_eyre::eyre::Result;
        use rusqlite::Connection;

        fn init_local_seeders(conn: &mut Connection) {
            init_data(conn).expect("Error al iniciar los seeders");
            SetzeRepo::bulk_insert(
                conn,
                &[
                    NewSetzeSchema {
                        setze_spanisch: "123".into(),
                        setze_deutsch: "456".into(),
                        niveau_id: 1,
                        thema: "789".into(),
                    },
                    NewSetzeSchema {
                        setze_spanisch: "123".into(),
                        setze_deutsch: "456".into(),
                        niveau_id: 1,
                        thema: "789".into(),
                    },
                ],
            )
            .expect("Error al inicializar oraciones.");
        }

        fn run_bulk_insert_scenario<F>(caller: F)
        where
            F: FnOnce() -> Result<Vec<Schema>>,
        {
            let res_1 = caller().expect("La inserción no deberia fallar");

            assert_eq!(res_1.len(), 2);

            assert_eq!(res_1[0].id, 1);
            assert_eq!(res_1[0].setze_id, 1);
            assert!(res_1[0].result);

            assert_eq!(res_1[1].id, 2);
            assert_eq!(res_1[1].setze_id, 2);
            assert!(!res_1[1].result);

            let res_1 = placeholder_dates(res_1);
            insta::assert_debug_snapshot!(res_1);
        }

        #[test]
        fn test_bulk_insert() {
            let data = vec![
                New {
                    setze_id: 1,
                    result: true,
                },
                New {
                    setze_id: 2,
                    result: false,
                },
            ];
            let mut conn = setup_test_db().unwrap();
            init_local_seeders(&mut conn);
            run_bulk_insert_scenario(|| GeschichtlichSetzeRepo::bulk_insert(&mut conn, &data));
        }

        #[test]
        fn test_bulk_insert_and_update_tx() {
            let data = vec![
                New {
                    setze_id: 1,
                    result: true,
                },
                New {
                    setze_id: 2,
                    result: false,
                },
            ];
            let mut conn = setup_test_db().unwrap();
            init_local_seeders(&mut conn);
            let tx = conn.transaction().expect("");
            run_bulk_insert_scenario(|| GeschichtlichSetzeRepo::bulk_insert_tx(&tx, &data));
            tx.commit().expect("");
        }
    }
}
