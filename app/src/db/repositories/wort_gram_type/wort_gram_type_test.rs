#[cfg(test)]
mod test_worte_gram_type_repo {
    use rusqlite::Connection;

    use crate::{
        db::{
            schemas::wort_gram_type::{
                InputWortGramType, SchemaWortGramType, SnapshotWortGramType,
            },
            wort_gram_type::WortGramTypeRepo,
        },
        helpers::error_handler::DbError,
        test_utils::scenarios::{scenario_wort, scenario_wort_gram_type},
    };

    fn assert_iter(res: &[SchemaWortGramType], data: &[InputWortGramType]) {
        assert_eq!(res.len(), data.len());

        for (i, satz) in data.iter().enumerate() {
            assert_eq!(res[i].id_worte, satz.id_worte);
            assert_eq!(res[i].id_gram_type, satz.id_gram_type);
            assert!(res[i].deleted_at.is_none());
        }
    }

    mod bulk_insert {
        use crate::db::{schemas::wort::InputWort, worte::WorteRepo};

        use super::*;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            // We need to change this, in other case the test will failed, cause there are information
            // added that it is not in the ScenarioWortGramType
            let data: Vec<_> = scenario_wort()
                .initial
                .into_iter()
                .map(|d| InputWort {
                    gram_type: vec![],
                    ..s
                })
                .collect();
            WorteRepo::bulk_insert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_gram_type().initial;
            let res = WortGramTypeRepo::bulk_insert(&mut conn, &[])?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotWortGramType> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortGramType::bulk_insert] - empty", snapshot);

            Ok(())
        }

        #[test]
        fn insert() -> Result<(), DbError> {
            let mut conn = init_conn()?;
            let data = scenario_wort_gram_type().initial;

            let res = WortGramTypeRepo::bulk_insert(&mut conn, &data)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotWortGramType> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortGramType::bulk_insert] - insert", snapshot);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let data = scenario_wort_gram_type().initial;
            let err = WortGramTypeRepo::bulk_insert(&mut conn, &data).unwrap_err();

            assert!(err.sql.is_some(), "expected DbError.sql to be Some(sql)");
            assert!(
                err.message.to_lowercase().contains("no such table")
                    || format!("{:?}", err)
                        .to_lowercase()
                        .contains("no such table"),
                "unexpected error: {err:?}"
            );

            Ok(())
        }
    }

    mod fetch_by_wort_id {
        use super::*;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            let data = scenario_wort().initial;
            WorteRepo::bulk_insert(&mut conn, &data)?;

            let data = scenario_wort_gram_type().initial;
            WortGramTypeRepo::bulk_insert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let conn = init_conn()?;

            let res = WortGramTypeRepo::fetch_by_wort_id(&conn, &[])?;

            assert_iter(&res, &[]);

            let snapshot: Vec<SnapshotWortGramType> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortGramType::fetch_by_wort_id] - empty", snapshot);

            Ok(())
        }

        #[test]
        fn happy_path() -> Result<(), DbError> {
            let conn = init_conn()?;

            let id_fetch = vec![1, 2];
            let res = WortGramTypeRepo::fetch_by_wort_id(&conn, &id_fetch)?;
            let data: Vec<_> = scenario_wort_gram_type()
                .initial
                .into_iter()
                .filter(|d| id_fetch.contains(&d.id_worte))
                .collect();

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotWortGramType> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!(
                "[WortGramType::fetch_by_wort_id] - happy_path",
                snapshot
            );

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = WortGramTypeRepo::fetch_by_wort_id(&mut conn, &[1]).unwrap_err();

            assert!(err.sql.is_some(), "expected DbError.sql to be Some(sql)");
            assert!(
                err.message.to_lowercase().contains("no such table")
                    || format!("{:?}", err)
                        .to_lowercase()
                        .contains("no such table"),
                "unexpected error: {err:?}"
            );

            Ok(())
        }
    }

    mod fetch_all_worte_id {
        use super::*;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            let data = scenario_wort().initial;
            WorteRepo::bulk_insert(&mut conn, &data)?;

            let data = scenario_wort_gram_type().initial;
            WortGramTypeRepo::bulk_insert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn happy_path() -> Result<(), DbError> {
            let conn = init_conn()?;

            let data = scenario_wort_gram_type().initial;
            let mut res = WortGramTypeRepo::fetch_all_worte_id(&conn, 100, 0)?;

            let mut data_compared: Vec<i32> = data.into_iter().map(|w| w.id_worte).collect();
            data_compared.sort_unstable();
            data_compared.dedup();

            assert_eq!(vec_res, data_compared);

            insta::assert_debug_snapshot!(
                "[WortGramType::fetch_all_worte_id] - happy_path",
                vec_res
            );

            Ok(())
        }

        #[test]
        fn offset_logic() -> Result<(), DbError> {
            let conn = init_conn()?;

            let limit: usize = 1;
            let mut last_id: i32 = 0;

            let mut vec_res: Vec<i32> = vec![];
            loop {
                let mut res = WortGramTypeRepo::fetch_all_worte_id(&conn, limit, last_id)?;

                if res.is_empty() {
                    break;
                }

                last_id = res.last().unwrap().clone();
                vec_res.append(&mut res);
            }

            let data = scenario_wort_gram_type().initial;
            let mut data_compared: Vec<i32> = data.into_iter().map(|w| w.id_worte).collect();
            data_compared.sort_unstable();
            data_compared.dedup();

            assert_eq!(vec_res, data_compared);

            insta::assert_debug_snapshot!(
                "[WortGramType::fetch_all_worte_id] - offset_logic",
                vec_res
            );

            Ok(())
            //
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = WortGramTypeRepo::fetch_all_worte_id(&mut conn, 100, 0).unwrap_err();

            assert!(err.sql.is_some(), "expected DbError.sql to be Some(sql)");
            assert!(
                err.message.to_lowercase().contains("no such table")
                    || format!("{:?}", err)
                        .to_lowercase()
                        .contains("no such table"),
                "unexpected error: {err:?}"
            );

            Ok(())
        }
    }

    mod delete_by_wort_id {
        use super::*;

        use crate::db::traits::FromSql;
        use rusqlite::params_from_iter;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            let data = scenario_wort().initial;
            WorteRepo::bulk_insert(&mut conn, &data)?;

            let data = scenario_wort_gram_type().initial;
            WortGramTypeRepo::bulk_insert(&mut conn, &data)?;

            Ok(conn)
        }

        fn fetch_all_data(conn: &Connection) -> Result<Vec<SchemaWortGramType>, DbError> {
            let sql = "
                SELECT 
                    id_worte, id_gram_type, created_at, deleted_at
                FROM worte_gram_type wgt
                ORDER BY wgt.id_worte;
            ";

            let mut stmt = conn.prepare(sql).map_err(DbError::with_sql(sql))?;
            let raw = stmt
                .query(params_from_iter(ids))
                .map_err(DbError::with_sql(sql))?
                .mapped(SchemaWortGramType::from_sql)
                .collect::<Result<Vec<_>, _>>()
                .map_err(DbError::with_sql(sql))?;

            Ok(raw)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            // Check if it doesn't remove anything from DB
            let fetch_before = fetch_all_data(&conn)?;

            let res = WortGramTypeRepo::delete_by_wort_id(&mut conn, &[])?;
            assert_eq!(res, 0);

            // Check if it doesn't remove anything from DB
            let fetch_after = fetch_all_data(&conn)?;

            assert_eq!(fetch_before, fetch_after);

            insta::assert_debug_snapshot!("[WortGramType::delete_by_wort_id] - empty", res);

            Ok(())
        }

        #[test]
        fn delete_all() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_gram_type().initial;
            let ids_remove: Vec<i32> = data.into_iter().map(|w| w.id_worte).collect();
            let res = WortGramTypeRepo::delete_by_wort_id(&mut conn, &ids_remove)?;
            assert_eq!(res, ids_remove.len());

            // Check if it doesn't remove anything from DB
            let fetch_after = fetch_all_data(&conn)?;
            assert_eq!(fetch_after, vec![]);

            insta::assert_debug_snapshot!("[WortGramType::delete_by_wort_id] - delete_all", res);

            Ok(())
        }

        #[test]
        fn delete_one() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            // Check if it doesn't remove anything from DB
            let fetch_before = fetch_all_data(&conn)?;

            let mut data = scenario_wort_gram_type().initial;
            let id_remove: i32 = data.pop().unwrap().id_worte;
            let res = WortGramTypeRepo::delete_by_wort_id(&mut conn, &[id_remove])?;

            // We get the rows that should be removed
            let rows_affected: Vec<_> = fetch_before
                .iter()
                .filter(|f| f.id_worte == id_remove)
                .collect();

            assert_eq!(res, rows_affected.len());

            // Check if it doesn't remove anything from DB
            let fetch_after = fetch_all_data(&conn)?;

            // Convert fetch_before to only removing id_remove, for checking if it doesn't affected
            // anything else
            let fetch_before: Vec<_> = fetch_before
                .into_iter()
                .filter(|f| f.id_worte != id_remove)
                .collect();

            assert_eq!(fetch_before, fetch_after);

            insta::assert_debug_snapshot!("[WortGramType::delete_by_wort_id] - delete_one", res);

            Ok(())
        }
    }
}
