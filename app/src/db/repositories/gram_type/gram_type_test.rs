#[cfg(test)]
mod test_gram_type_repo {

    use crate::{
        db::{
            gram_type::GramTypeRepo,
            schemas::gram_type::{GramTypeSchema as Schema, NewGramTypeSchema as New},
            setup_test_db,
        },
        impl_test_helpers_for_schema,
    };

    impl_test_helpers_for_schema!(
        schema = Schema,
        snapshot = Snapshot,
        fields = [ id: i32, code: String, name: String ],
        placeholders = [ created_at, deleted_at ]
    );

    mod bulk_upsert {
        use std::{thread, time::Duration};

        use super::*;
        use color_eyre::eyre::Result;
        use rusqlite::Connection;

        #[derive(Clone)]
        struct Scenario<T> {
            initial: Vec<T>,
            update: Vec<T>,
        }

        fn scenario() -> Scenario<New> {
            Scenario {
                initial: vec![
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
                ],
                update: vec![
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
                ],
            }
        }

        fn run_bulk_upsert_scenario<F>(insert_fn: F, sc: Scenario<New>)
        where
            F: Fn(&mut Connection, &[New]) -> Result<Vec<Schema>>,
        {
            let mut conn = setup_test_db().unwrap();

            let res_1 = insert_fn(&mut conn, &sc.initial).expect("La inserción no deberia fallar");

            assert_eq!(res_1.len(), 2);
            res_1[0].assert_fields(1, "123".into(), "456".into());
            res_1[1].assert_fields(2, "987".into(), "654".into());
            insta::assert_debug_snapshot!("after_insert", res_1.snapshot());

            thread::sleep(Duration::from_millis(100));
            let res_2 = insert_fn(&mut conn, &sc.update).expect("La inserción no deberia fallar");

            assert_eq!(res_2.len(), 2);
            res_2[0].assert_fields(1, "abc".into(), "def".into());
            res_2[1].assert_fields(2, "987".into(), "zyw".into());
            insta::assert_debug_snapshot!("after_update", res_2.snapshot());
        }

        #[test]
        fn test_bulk_upsert() {
            let sc = scenario();
            run_bulk_upsert_scenario(|conn, data| GramTypeRepo::bulk_insert(conn, data), sc);
        }
    }
}
