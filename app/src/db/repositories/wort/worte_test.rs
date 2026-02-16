#[cfg(test)]
mod test_worte_repo {
    use std::collections::HashMap;

    use crate::{
        db::{
            schemas::{
                init_schemas,
                wort::{InputWort, SchemaWort, SnapshotWort},
                wort_gram_type::{SchemaWortGramType, SnapshotWortGramType},
            },
            seeders::init_data,
            wort::WortRepo,
        },
        helpers::{error_handler::DbError, time::string_2_datetime},
        test_utils::scenarios::scenario_wort,
    };
    use rusqlite::Connection;

    fn assert_iter(res: &[SchemaWort], hash: &HashMap<i32, Vec<SchemaWort>>, data: &[InputWort]) {
        assert_eq!(res.len(), data.len());

        for (i, wort) in data.iter().enumerate() {
            assert!(res[i].id > 0);

            assert_eq!(res[i].gender_id, wort.gender.map(|d| d.id()));
            assert_eq!(res[i].worte_de, wort.wort_de);
            assert_eq!(res[i].worte_es, wort.wort_es);
            assert_eq!(res[i].plural, wort.plural);
            assert_eq!(res[i].niveau_id, wort.niveau.id());
            assert_eq!(res[i].example_de, wort.example_de);
            assert_eq!(res[i].example_es, wort.example_es);
            assert_eq!(res[i].verb_aux, wort.verb_aux);
            assert_eq!(res[i].trennbar, wort.trennbar);
            assert_eq!(res[i].reflexiv, wort.reflexiv);

            let grams = hash.remove(res[i].id).unwrap();
            let mut ids_grams: Vec<i32> = grams.into_iter().map(|d| d.id).collect();
            ids_grams.sort_unstable();

            let mut ids_data: Vec<i32> = data.iter().map(|d| d.gram_type).collect();
            ids_data.sort_unstable();

            assert_eq!(ids_grams, ids_data);

            assert!(string_2_datetime(res[i].created_at).is_ok());
            assert!(res[i].deleted_at.is_none());
        }
    }

    mod bulk_insert {
        use super::*;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let (res, hash) = WortRepo::bulk_insert(&mut conn, &[])?;

            assert_iter(&res, &hash, &[]);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::bulk_insert] - empty (res)", ss);

            let ss: Vec<SchemaWortGramType> = hash.into_values().flatten().collect(); // Make all the vecs into the same level
            let ss: Vec<SnapshotWortGramType> = ss.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::bulk_insert] - empty (hash)", ss);

            Ok(())
        }

        #[test]
        fn insert() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort().initial;
            let (res, hash) = WortRepo::bulk_insert(&mut conn, &data)?;

            assert_iter(&res, &hash, &data);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::bulk_insert] - insert (res)", ss);

            let ss: Vec<SchemaWortGramType> = hash.into_values().flatten().collect(); // Make all the vecs into the same level
            let ss: Vec<SnapshotWortGramType> = ss.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::bulk_insert] - insert (hash)", ss);
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let data = scenario_wort().initial;
            let err = WortRepo::bulk_insert(&mut conn, &data).unwrap_err();

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

    mod bulk_update {
        use super::*;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            let data = scenario_wort().initial;
            WortRepo::bulk_insert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let (res, hash) = WortRepo::bulk_update(&mut conn, &[])?;

            assert_iter(&res, &hash, &[]);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::bulk_update] - empty (res)", ss);

            let ss: Vec<SchemaWortGramType> = hash.into_values().flatten().collect(); // Make all the vecs into the same level
            let ss: Vec<SnapshotWortGramType> = ss.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::bulk_update] - empty (hash)", ss);

            Ok(())
        }

        #[test]
        fn update() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort().update;
            let (res, hash) = WortRepo::bulk_update(&mut conn, &data)?;

            assert_iter(&res, &hash, &data);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::bulk_update] - update (res)", ss);

            let ss: Vec<SchemaWortGramType> = hash.into_values().flatten().collect(); // Make all the vecs into the same level
            let ss: Vec<SnapshotWortGramType> = ss.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::bulk_update] - update (hash)", ss);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let data = scenario_wort().initial;
            let err = WortRepo::bulk_update(&mut conn, &data).unwrap_err();

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

    mod fetch_by_id {
        use super::*;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            let data = scenario_wort().initial;
            WortRepo::bulk_insert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let (res, hash) = WortRepo::fetch_by_id(&mut conn, &[])?;

            assert_iter(&res, &hash, &[]);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_id] - empty (res)", ss);

            let ss: Vec<SchemaWortGramType> = hash.into_values().flatten().collect(); // Make all the vecs into the same level
            let ss: Vec<SnapshotWortGramType> = ss.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_id] - empty (hash)", ss);

            Ok(())
        }

        #[test]
        fn data() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let (res, hash) = WortRepo::fetch_by_id(&mut conn, &[1, 2])?;

            let data: Vec<_> = scenario_wort().initial.into_iter().take(2).collect();
            assert_iter(&res, &hash, &data);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_id] - data (res)", ss);

            let ss: Vec<SchemaWortGramType> = hash.into_values().flatten().collect(); // Make all the vecs into the same level
            let ss: Vec<SnapshotWortGramType> = ss.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_id] - data (hash)", ss);

            Ok(())
        }

        #[test]
        fn not_exists() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let (res, hash) = WortRepo::fetch_by_id(&mut conn, &[-1])?;

            assert_iter(&res, &hash, &[]);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_id] - not_exists (res)", ss);

            let ss: Vec<SchemaWortGramType> = hash.into_values().flatten().collect(); // Make all the vecs into the same level
            let ss: Vec<SnapshotWortGramType> = ss.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_id] - not_exists (hash)", ss);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = WortRepo::fetch_by_id(&mut conn, &[1]).unwrap_err();

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

    mod fetch_all_ids {
        use super::*;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let res = WortRepo::fetch_all_ids(&conn, 100, 0)?;

            assert_eq!(res, []);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_all_ids] - empty", ss);

            Ok(())
        }

        #[test]
        fn with_data() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort().initial;
            WortRepo::bulk_insert(&mut conn, &data)?;

            let res = WortRepo::fetch_all_ids(&conn, 100_000, 0)?;

            let data: Vec<_> = data
                .into_iter()
                .enumerate()
                .map(|(id, _)| (id + 1) as i32)
                .collect();

            assert_eq!(res, data);

            insta::assert_debug_snapshot!("[WortRepo::fetch_all_ids] - with_data", res);

            Ok(())
        }

        #[test]
        fn offset_logic() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort().initial;
            WortRepo::bulk_insert(&mut conn, &data)?;

            let limit = 1;
            let mut last_id = 0;
            let mut vec_out = vec![];

            loop {
                let mut res = WortRepo::fetch_all_ids(&conn, limit, last_id)?;

                if res.is_empty() {
                    break;
                }

                last_id = res.iter().last().unwrap();
                vec_out.append(&mut res);
            }

            let data: Vec<_> = data
                .into_iter()
                .enumerate()
                .map(|(id, _)| (id + 1) as i32)
                .collect();

            assert_eq!(vec_out, data);

            insta::assert_debug_snapshot!("[WortRepo::fetch_all_ids] - offset_logic", vec_out);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = WortRepo::fetch_all_ids(&mut conn, 100, 0).unwrap_err();

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

    mod fetch_by_wort {
        use super::*;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let (res, hash) = WortRepo::fetch_by_wort(&conn, &[])?;

            assert_iter(&res, &hash, &[]);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_wort] - empty", ss);

            Ok(())
        }

        #[test]
        fn all_data() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort().initial;
            WortRepo::bulk_insert(&mut conn, &data)?;

            let data_fetch: Vec<_> = data
                .iter()
                .map(|d| (d.worte_es.clone(), d.worte_de.clone()))
                .collect();
            let (res, hash) = WortRepo::fetch_by_id(&mut conn, &data_fetch)?;

            assert_iter(&res, &hash, &data);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_wort] - all_data (res)", ss);

            let ss: Vec<SchemaWortGramType> = hash.into_values().flatten().collect(); // Make all the vecs into the same level
            let ss: Vec<SnapshotWortGramType> = ss.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_wort] - all_data (hash)", ss);

            Ok(())
        }

        #[test]
        fn one_row() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort().initial;
            WortRepo::bulk_insert(&mut conn, &data)?;

            let data_fetch: Vec<_> = data
                .iter()
                .map(|d| (d.worte_es.clone(), d.worte_de.clone()))
                .take(1)
                .collect();
            let (res, hash) = WortRepo::fetch_by_id(&mut conn, &data_fetch)?;

            let data: Vec<_> = data.into_iter().take(1).collect();
            assert_iter(&res, &hash, &data);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_wort] - one_row (res)", ss);

            let ss: Vec<SchemaWortGramType> = hash.into_values().flatten().collect(); // Make all the vecs into the same level
            let ss: Vec<SnapshotWortGramType> = ss.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_wort] - one_row (hash)", ss);

            Ok(())
        }

        #[test]
        fn not_exists() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort().initial;
            WortRepo::bulk_insert(&mut conn, &data)?;

            let data_fetch: Vec<_> = vec![(
                String::from("NOT_VALID_VALUE"),
                String::from("NOT_VALID_VALUE"),
            )];

            let (res, hash) = WortRepo::fetch_by_id(&mut conn, &data_fetch)?;

            assert_iter(&res, &hash, &[]);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_wort] - not_exists (res)", ss);

            let ss: Vec<SchemaWortGramType> = hash.into_values().flatten().collect(); // Make all the vecs into the same level
            let ss: Vec<SnapshotWortGramType> = ss.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_wort] - not_exists (hash)", ss);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = WortRepo::fetch_by_wort(&mut conn, &[]).unwrap_err();

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
}
