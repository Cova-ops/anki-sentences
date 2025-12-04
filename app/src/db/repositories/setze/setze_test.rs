use crate::db::setup_test_db;

#[cfg(test)]
mod test_setze_repo {

    use crate::db::{
        schemas::setze::{NewSetzeSchema as New, SetzeSchema as Schema},
        setze::SetzeRepo,
    };

    use super::*;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Snapshot {
        id: i32,
        setze_spanisch: String,
        setze_deutsch: String,
        schwirigkeit_id: SchwirigkeitListeSnapshot,
        thema: String,

        created_at: String,
        deleted_at: String,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct SchwirigkeitListeSnapshot {
        id: i32,
        schwirigkeit: String,
        created_at: String,
        deleted_at: String,
    }

    fn placeholder_dates(data: Vec<Schema>) -> Vec<Snapshot> {
        data.into_iter()
            .map(|d| Snapshot {
                id: d.id,
                setze_spanisch: d.setze_spanisch,
                setze_deutsch: d.setze_deutsch,
                schwirigkeit_id: SchwirigkeitListeSnapshot {
                    id: d.schwirigkeit_id.id,
                    schwirigkeit: d.schwirigkeit_id.schwirigkeit,
                    created_at: "<created_at>".into(),
                    deleted_at: "<deleted_at>".into(),
                },
                thema: d.thema,

                created_at: "<created_at>".into(),
                deleted_at: "<deleted_at>".into(),
            })
            .collect()
    }

    mod insert_update {
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
            assert_eq!(res_1[0].setze_spanisch, "Hola");
            assert_eq!(res_1[0].setze_deutsch, "Hallo");
            assert_eq!(res_1[0].schwirigkeit_id.id, 1);
            assert_eq!(res_1[0].thema, "Thema 1");

            assert_eq!(res_1[1].id, 2);
            assert_eq!(res_1[1].setze_spanisch, "Adios");
            assert_eq!(res_1[1].setze_deutsch, "Tschüss");
            assert_eq!(res_1[1].schwirigkeit_id.id, 2);
            assert_eq!(res_1[1].thema, "Thema 2");

            let res_1 = placeholder_dates(res_1);
            insta::assert_debug_snapshot!(res_1);
        }

        #[test]
        fn test_bulk_insert() {
            let data_1 = vec![
                New {
                    setze_spanisch: "Hola".into(),
                    setze_deutsch: "Hallo".into(),
                    schwirigkeit_id: 1,
                    thema: "Thema 1".into(),
                },
                New {
                    setze_spanisch: "Adios".into(),
                    setze_deutsch: "Tschüss".into(),
                    schwirigkeit_id: 2,
                    thema: "Thema 2".into(),
                },
            ];
            run_bulk_insert_update_scenario(|conn| SetzeRepo::bulk_insert(conn, &data_1));
        }

        #[test]
        fn test_bulk_insert_and_update_tx() {
            let data_1 = vec![
                New {
                    setze_spanisch: "Hola".into(),
                    setze_deutsch: "Hallo".into(),
                    schwirigkeit_id: 1,
                    thema: "Thema 1".into(),
                },
                New {
                    setze_spanisch: "Adios".into(),
                    setze_deutsch: "Tschüss".into(),
                    schwirigkeit_id: 2,
                    thema: "Thema 2".into(),
                },
            ];
            run_bulk_insert_update_scenario(|conn| {
                let tx = conn.transaction()?;
                let out = SetzeRepo::bulk_insert_tx(&tx, &data_1)?;
                tx.commit()?;
                Ok(out)
            });
        }
    }

    mod fetch {
        use color_eyre::eyre::Result;
        use rusqlite::Connection;

        use super::*;
        use crate::db::{seeders::init_data, setup_test_db, setze::SetzeRepo};

        fn init_data_local(conn: &mut Connection) -> Result<()> {
            init_data(conn)?;
            let data_1 = vec![
                New {
                    setze_spanisch: "Hola".into(),
                    setze_deutsch: "Hallo".into(),
                    schwirigkeit_id: 1,
                    thema: "Thema 1".into(),
                },
                New {
                    setze_spanisch: "Adios".into(),
                    setze_deutsch: "Tschüss".into(),
                    schwirigkeit_id: 2,
                    thema: "Thema 2".into(),
                },
            ];
            SetzeRepo::bulk_insert(conn, &data_1)?;
            Ok(())
        }

        #[test]
        fn test_fetch_by_id() {
            let mut conn = setup_test_db().expect("Error al crear db test");
            init_data_local(&mut conn).expect("Error al iniciar data test");

            let res = SetzeRepo::fetch_by_id(&conn, &[]).expect("Error al hacer fetch");
            assert_eq!(res.len(), 0);
            insta::assert_debug_snapshot!(res);

            let res = SetzeRepo::fetch_by_id(&conn, &[1, 2]).expect("Error al hacer fetch");
            assert_eq!(res.len(), 2);

            assert_eq!(res[0].id, 1);
            assert_eq!(res[0].setze_spanisch, "Hola");
            assert_eq!(res[0].setze_deutsch, "Hallo");
            assert_eq!(res[0].schwirigkeit_id.id, 1);
            assert_eq!(res[0].thema, "Thema 1");

            assert_eq!(res[1].id, 2);
            assert_eq!(res[1].setze_spanisch, "Adios");
            assert_eq!(res[1].setze_deutsch, "Tschüss");
            assert_eq!(res[1].schwirigkeit_id.id, 2);
            assert_eq!(res[1].thema, "Thema 2");

            let res = placeholder_dates(res);
            insta::assert_debug_snapshot!(res);

            let res = SetzeRepo::fetch_by_id(&conn, &[99]).expect("Error al hacer fetch");
            assert_eq!(res.len(), 0);
            insta::assert_debug_snapshot!(res);
        }

        #[test]
        fn test_fetch_where_thema() {
            let mut conn = setup_test_db().expect("Error al crear db test");
            init_data_local(&mut conn).expect("Error al iniciar data test");

            let res =
                SetzeRepo::fetch_where_thema(&conn, &[], 100, 0).expect("Error al hacer fetch");
            assert_eq!(res.len(), 0);
            insta::assert_debug_snapshot!(res);

            let res =
                SetzeRepo::fetch_where_thema(&conn, &["Thema 1".into(), "Thema 2".into()], 100, 0)
                    .expect("Error al hacer fetch");
            assert_eq!(res.len(), 2);

            assert_eq!(res[0].id, 1);
            assert_eq!(res[0].setze_spanisch, "Hola");
            assert_eq!(res[0].setze_deutsch, "Hallo");
            assert_eq!(res[0].schwirigkeit_id.id, 1);
            assert_eq!(res[0].thema, "Thema 1");

            assert_eq!(res[1].id, 2);
            assert_eq!(res[1].setze_spanisch, "Adios");
            assert_eq!(res[1].setze_deutsch, "Tschüss");
            assert_eq!(res[1].schwirigkeit_id.id, 2);
            assert_eq!(res[1].thema, "Thema 2");

            let res = placeholder_dates(res);
            insta::assert_debug_snapshot!(res);

            let res = SetzeRepo::fetch_where_thema(&conn, &["Thema 99".into()], 100, 0)
                .expect("Error al hacer fetch");
            assert_eq!(res.len(), 0);
            insta::assert_debug_snapshot!(res);

            let res =
                SetzeRepo::fetch_where_thema(&conn, &["Thema 1".into(), "Thema 2".into()], 1, 0)
                    .expect("Error al hacer fetch");
            assert_eq!(res.len(), 1);

            assert_eq!(res[0].id, 1);
            assert_eq!(res[0].setze_spanisch, "Hola");
            assert_eq!(res[0].setze_deutsch, "Hallo");
            assert_eq!(res[0].schwirigkeit_id.id, 1);
            assert_eq!(res[0].thema, "Thema 1");

            let res = placeholder_dates(res);
            insta::assert_debug_snapshot!(res);

            let res =
                SetzeRepo::fetch_where_thema(&conn, &["Thema 1".into(), "Thema 2".into()], 1, 1)
                    .expect("Error al hacer fetch");
            assert_eq!(res.len(), 1);

            assert_eq!(res[0].id, 2);
            assert_eq!(res[0].setze_spanisch, "Adios");
            assert_eq!(res[0].setze_deutsch, "Tschüss");
            assert_eq!(res[0].schwirigkeit_id.id, 2);
            assert_eq!(res[0].thema, "Thema 2");

            let res = placeholder_dates(res);
            insta::assert_debug_snapshot!(res);
        }

        #[test]
        fn test_fetch_id_schwirig_thema() {
            let mut conn = setup_test_db().expect("Error al crear db test");
            init_data_local(&mut conn).expect("Error al iniciar data test");

            let res =
                SetzeRepo::fetch_id_schwirig_thema(&conn, None).expect("Error al hacer fetch");

            assert_eq!(res.len(), 1);
            assert_eq!(res[0], 2);

            insta::assert_debug_snapshot!(res);

            let res = SetzeRepo::fetch_id_schwirig_thema(&conn, Some(&["Thema 1".into()]))
                .expect("Error al hacer fetch");

            assert_eq!(res.len(), 0);
            insta::assert_debug_snapshot!(res);

            let res = SetzeRepo::fetch_id_schwirig_thema(&conn, Some(&["Thema 2".into()]))
                .expect("Error al hacer fetch");

            assert_eq!(res.len(), 1);
            assert_eq!(res[0], 2);

            insta::assert_debug_snapshot!(res);
        }

        #[test]
        fn test_fetch_all_only_ids() {
            let mut conn = setup_test_db().expect("Error al crear db test");
            init_data_local(&mut conn).expect("Error al iniciar data test");

            let res = SetzeRepo::fetch_all_only_ids(&conn).expect("Error al hacer fetch");

            assert_eq!(res.len(), 2);
            assert_eq!(res[0], 1);
            assert_eq!(res[1], 2);

            insta::assert_debug_snapshot!(res);
        }

        #[test]
        fn test_fetch_all_themas() {
            let mut conn = setup_test_db().expect("Error al crear db test");
            init_data_local(&mut conn).expect("Error al iniciar data test");

            let res = SetzeRepo::fetch_all_themas(&conn).expect("Error al hacer fetch");

            assert_eq!(res.len(), 2);
            assert_eq!(res[0], "Thema 1");
            assert_eq!(res[1], "Thema 2");

            insta::assert_debug_snapshot!(res);
        }
    }
}
