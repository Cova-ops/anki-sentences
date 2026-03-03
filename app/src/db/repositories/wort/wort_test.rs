#[cfg(test)]
mod test_worte_repo {
    use crate::{
        db::{
            schemas::{
                init_schemas,
                wort::{InputWort, SchemaWort, SnapshotWort},
                wort_gram_type::SchemaWortGramType,
            },
            seeders::init_data,
            wort::WortRepo,
        },
        helpers::{error_handler::DbError, time::string_2_datetime},
        test_utils::scenarios::scenario_wort,
    };
    use rusqlite::Connection;

    fn assert_iter(res: &[(SchemaWort, Vec<SchemaWortGramType>)], data: &[InputWort]) {
        assert_eq!(res.len(), data.len());

        for (i, wort) in data.iter().enumerate() {
            let (res_wort, res_grams) = &res[i];

            assert!(res_wort.id > 0);

            assert_eq!(res_wort.gender_id, wort.gender.map(|d| d.id()));
            assert_eq!(res_wort.worte_de, wort.worte_de);
            assert_eq!(res_wort.worte_es, wort.worte_es);
            assert_eq!(res_wort.plural, wort.plural);
            assert_eq!(res_wort.niveau_id, wort.niveau.id());
            assert_eq!(res_wort.example_de, wort.example_de);
            assert_eq!(res_wort.example_es, wort.example_es);
            assert_eq!(res_wort.verb_aux, wort.verb_aux);
            assert_eq!(res_wort.trennbar, wort.trennbar);
            assert_eq!(res_wort.reflexiv, wort.reflexiv);

            let data_grams = &wort.gram_type;

            assert_eq!(res_grams.len(), data_grams.len());
            for (data_gram, res_gram) in data_grams.iter().zip(res_grams.iter()) {
                assert_eq!(res_wort.id, res_gram.id_worte);
                assert_eq!(data_gram.id(), res_gram.id_gram_type);
            }

            assert!(string_2_datetime(&res_wort.created_at).is_ok());
            assert!(res_wort.deleted_at.is_none());
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

            let res = WortRepo::bulk_insert(&mut conn, &[])?;

            assert_iter(&res, &[]);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::bulk_insert] - empty", ss);

            Ok(())
        }

        #[test]
        fn insert() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort().initial;
            let res = WortRepo::bulk_insert(&mut conn, &data)?;

            assert_iter(&res, &data);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::bulk_insert] - insert", ss);

            Ok(())
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

            let res = WortRepo::bulk_update(&mut conn, &[])?;

            assert_iter(&res, &[]);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::bulk_update] - empty", ss);

            Ok(())
        }

        #[test]
        fn update() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort().update_id;
            let res = WortRepo::bulk_update(&mut conn, &data)?;

            let data_without_id: Vec<_> =
                scenario_wort().update_id.into_iter().map(|d| d.1).collect();

            assert_iter(&res, &data_without_id);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::bulk_update] - update", ss);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let data = scenario_wort().update_id;
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

            let res = WortRepo::fetch_by_id(&mut conn, &[])?;

            assert_iter(&res, &[]);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_id] - empty", ss);

            Ok(())
        }

        #[test]
        fn data() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let res = WortRepo::fetch_by_id(&mut conn, &[1, 2])?;

            let data: Vec<_> = scenario_wort().initial.into_iter().take(2).collect();
            assert_iter(&res, &data);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_id] - data", ss);

            Ok(())
        }

        #[test]
        fn not_exists() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let res = WortRepo::fetch_by_id(&mut conn, &[-1])?;

            assert_iter(&res, &[]);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_id] - not_exists", ss);

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

    mod fetch_one {
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
        fn data() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let res = WortRepo::fetch_one(&mut conn, 1)?.unwrap();

            let data: _ = scenario_wort().initial.into_iter().next().unwrap();
            assert_iter(&[res.clone()], &[data]);

            let ss: SnapshotWort = res.into();
            insta::assert_debug_snapshot!("[WortRepo::fetch_one] - data", ss);

            Ok(())
        }

        #[test]
        fn not_exists() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let res = WortRepo::fetch_one(&mut conn, -1)?;

            assert!(res.is_none());

            insta::assert_debug_snapshot!("[WortRepo::fetch_one] - not_exists", res);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = WortRepo::fetch_one(&mut conn, 1).unwrap_err();

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
            let conn = init_conn()?;

            let res: Vec<i32> = WortRepo::fetch_all_ids(&conn, 100, 0)?;

            assert_eq!(res, Vec::<i32>::new());

            insta::assert_debug_snapshot!("[WortRepo::fetch_all_ids] - empty", res);

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

                last_id = *res.iter().last().unwrap();
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
            let conn = init_conn()?;

            let res = WortRepo::fetch_by_wort(&conn, &[])?;

            assert_iter(&res, &[]);

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
            let res = WortRepo::fetch_by_wort(&mut conn, &data_fetch)?;

            assert_iter(&res, &data);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_wort] - all_data", ss);

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
            let res = WortRepo::fetch_by_wort(&mut conn, &data_fetch)?;

            let data: Vec<_> = data.into_iter().take(1).collect();
            assert_iter(&res, &data);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_wort] - one_row", ss);

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

            let res = WortRepo::fetch_by_wort(&mut conn, &data_fetch)?;

            assert_iter(&res, &[]);

            let ss: Vec<SnapshotWort> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortRepo::fetch_by_wort] - not_exists (res)", ss);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err =
                WortRepo::fetch_by_wort(&mut conn, &[(format!("a"), format!("b"))]).unwrap_err();

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
