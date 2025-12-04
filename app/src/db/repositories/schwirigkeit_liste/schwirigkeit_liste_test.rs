use crate::db::setup_test_db;

#[cfg(test)]
mod test_schwirigkeit_liste_repo {

    use crate::db::{
        schemas::schwirigkeit_liste::{
            NewSchwirigkeitListeSchema as New, SchwirigkeitListeSchema as Schema,
        },
        schwirigkeit_liste::SchwirigkeitListeRepo,
    };

    use super::*;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Snapshot {
        id: i32,
        schwirigkeit: String,
        created_at: String,
        deleted_at: String,
    }

    fn placeholder_dates(data: Vec<Schema>) -> Vec<Snapshot> {
        data.into_iter()
            .map(|d| Snapshot {
                id: d.id,
                schwirigkeit: d.schwirigkeit,
                created_at: "<created_at>".into(),
                deleted_at: "<deleted_at>".into(),
            })
            .collect()
    }

    mod bulk_insert {
        use color_eyre::eyre::Result;
        use rusqlite::Connection;

        use super::*;

        fn run_bulk_insert_update_scenario<F1, F2>(c1: F1, c2: F2)
        where
            F1: FnOnce(&mut Connection) -> Result<Vec<Schema>>,
            F2: FnOnce(&mut Connection) -> Result<Vec<Schema>>,
        {
            let mut conn = setup_test_db().unwrap();

            let res_1 = c1(&mut conn).expect("La inserción no debe fallar");

            assert_eq!(res_1.len(), 2);
            assert_eq!(res_1[0].id, 1);
            assert_eq!(res_1[0].schwirigkeit, "einfag");

            let res_1 = placeholder_dates(res_1);
            insta::assert_debug_snapshot!(res_1);

            let res_2 = c2(&mut conn).expect("La actualización no debe fallar");

            assert_eq!(res_2.len(), 2);
            assert_eq!(res_2[1].id, 2);
            assert_eq!(res_2[1].schwirigkeit, "schwirig");

            let res_2 = placeholder_dates(res_2);
            insta::assert_debug_snapshot!(res_2);
        }

        #[test]
        fn test_bulk_insert_and_update() {
            let data_1 = vec![
                New {
                    id: 1,
                    schwirigkeit: "einfag".into(),
                },
                New {
                    id: 2,
                    schwirigkeit: "normal".into(),
                },
            ];
            let data_2 = vec![
                New {
                    id: 1,
                    schwirigkeit: "einfag".into(),
                },
                New {
                    id: 2,
                    schwirigkeit: "schwirig".into(),
                },
            ];
            run_bulk_insert_update_scenario(
                |conn| SchwirigkeitListeRepo::bulk_insert(conn, &data_1),
                |conn| SchwirigkeitListeRepo::bulk_insert(conn, &data_2),
            );
        }

        #[test]
        fn test_bulk_insert_and_update_tx() {
            let data_1 = vec![
                New {
                    id: 1,
                    schwirigkeit: "einfag".into(),
                },
                New {
                    id: 2,
                    schwirigkeit: "normal".into(),
                },
            ];
            let data_2 = vec![
                New {
                    id: 1,
                    schwirigkeit: "einfag".into(),
                },
                New {
                    id: 2,
                    schwirigkeit: "schwirig".into(),
                },
            ];
            run_bulk_insert_update_scenario(
                |conn| {
                    let tx = conn.transaction()?;
                    let out = SchwirigkeitListeRepo::bulk_insert_tx(&tx, &data_1)?;
                    tx.commit()?;
                    Ok(out)
                },
                |conn| {
                    let tx = conn.transaction()?;
                    let out = SchwirigkeitListeRepo::bulk_insert_tx(&tx, &data_2)?;
                    tx.commit()?;
                    Ok(out)
                },
            );
        }
    }
}
