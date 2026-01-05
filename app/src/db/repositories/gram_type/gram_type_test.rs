#[cfg(test)]
mod test_gram_type_repo {
    use color_eyre::eyre::Result;
    use rusqlite::Connection;

    use crate::{
        db::{
            gram_type::GramTypeRepo,
            schemas::gram_type::{GramTypeSchema as Schema, NewGramTypeSchema as New},
            setup_test_db,
        },
        impl_test_helpers_for_schema,
        test_utils::{
            scenarios::{Scenario, gram_type::scenario_gram_type_schema},
            traits::{AssertEqFields, SnapshotFields},
        },
    };

    impl_test_helpers_for_schema!(
        schema = Schema,
        new = New,
        snapshot = Snapshot,
        fields = [ id: i32, code: String, name: String ],
        placeholders = [ created_at, deleted_at ]
    );

    mod bulk_upsert {

        use super::*;

        fn run_bulk_upsert_scenario<F>(insert_fn: F, sc: Scenario<New>)
        where
            F: Fn(&mut Connection, &[New]) -> Result<Vec<Schema>>,
        {
            let mut conn = setup_test_db().unwrap();

            let res_1 = insert_fn(&mut conn, &sc.initial).expect("La inserción no deberia fallar");
            res_1.assert_eq_fields(&sc.initial);
            insta::assert_debug_snapshot!("after_insert", res_1.snapshot());

            // thread::sleep(Duration::from_millis(100));
            let res_2 = insert_fn(&mut conn, &sc.update).expect("La inserción no deberia fallar");
            res_2.assert_eq_fields(&sc.update);
            insta::assert_debug_snapshot!("after_update", res_2.snapshot());
        }

        #[test]
        fn test_bulk_upsert() {
            let sc = scenario_gram_type_schema();
            run_bulk_upsert_scenario(|conn, data| GramTypeRepo::bulk_insert(conn, data), sc);
        }
    }
}
