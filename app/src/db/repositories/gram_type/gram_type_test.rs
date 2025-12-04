use crate::db::setup_test_db;

#[cfg(test)]
mod test_gram_type_repo {

    use crate::db::{
        gram_type::GramTypeRepo,
        schemas::gram_type::{GramTypeSchema as Schema, NewGramTypeSchema as New},
    };

    use super::*;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Snapshot {
        id: i32,
        code: String,
        name: String,
        created_at: String,
        deleted_at: String,
    }

    fn placeholder_dates(data: Vec<Schema>) -> Vec<Snapshot> {
        data.into_iter()
            .map(|d| Snapshot {
                id: d.id,
                code: d.code,
                name: d.name,
                created_at: "<created_at>".into(),
                deleted_at: "<deleted_at>".into(),
            })
            .collect()
    }

    mod bulk_insert {

        use super::*;
        use color_eyre::eyre::Result;
        use rusqlite::Connection;

        fn run_bulk_insert_update_scenario<F1, F2>(c1: F1, c2: F2)
        where
            F1: FnOnce(&mut Connection) -> Result<Vec<Schema>>,
            F2: FnOnce(&mut Connection) -> Result<Vec<Schema>>,
        {
            let mut conn = setup_test_db().unwrap();

            let res_1 = c1(&mut conn).expect("La inserción no deberia fallar");

            assert_eq!(res_1.len(), 2);

            assert_eq!(res_1[0].id, 1);
            assert_eq!(res_1[0].code, "123");
            assert_eq!(res_1[0].name, "456");

            assert_eq!(res_1[1].id, 2);
            assert_eq!(res_1[1].code, "987");
            assert_eq!(res_1[1].name, "654");

            let res_1 = placeholder_dates(res_1);
            insta::assert_debug_snapshot!(res_1);

            let res_2 = c2(&mut conn).expect("La inserción no deberia fallar");

            assert_eq!(res_2.len(), 2);

            assert_eq!(res_2[0].id, 1);
            assert_eq!(res_2[0].code, "abc");
            assert_eq!(res_2[0].name, "def");

            assert_eq!(res_2[1].id, 2);
            assert_eq!(res_2[1].code, "987");
            assert_eq!(res_2[1].name, "zyw");

            let res_2 = placeholder_dates(res_2);
            insta::assert_debug_snapshot!(res_2);
        }

        #[test]
        fn test_bulk_insert() {
            let data_1 = vec![
                New {
                    id: 1,
                    code: "123".into(),
                    name: "456".into(),
                },
                New {
                    id: 2,
                    code: "987".into(),
                    name: "654".into(),
                },
            ];
            let data_2 = vec![
                New {
                    id: 1,
                    code: "abc".into(),
                    name: "def".into(),
                },
                New {
                    id: 2,
                    code: "987".into(),
                    name: "zyw".into(),
                },
            ];
            run_bulk_insert_update_scenario(
                |conn| GramTypeRepo::bulk_insert(conn, &data_1),
                |conn| GramTypeRepo::bulk_insert(conn, &data_2),
            );
        }

        #[test]
        fn test_bulk_insert_and_update_tx() {
            let data_1 = vec![
                New {
                    id: 1,
                    code: "123".into(),
                    name: "456".into(),
                },
                New {
                    id: 2,
                    code: "987".into(),
                    name: "654".into(),
                },
            ];
            let data_2 = vec![
                New {
                    id: 1,
                    code: "abc".into(),
                    name: "def".into(),
                },
                New {
                    id: 2,
                    code: "987".into(),
                    name: "zyw".into(),
                },
            ];
            run_bulk_insert_update_scenario(
                |conn| {
                    let tx = conn.transaction()?;
                    let out = GramTypeRepo::bulk_insert_tx(&tx, &data_1)?;
                    tx.commit()?;
                    Ok(out)
                },
                |conn| {
                    let tx = conn.transaction()?;
                    let out = GramTypeRepo::bulk_insert_tx(&tx, &data_2)?;
                    tx.commit()?;
                    Ok(out)
                },
            );
        }
    }
}
