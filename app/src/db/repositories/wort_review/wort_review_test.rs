#[cfg(test)]
mod test_worte_review_repo {
    use crate::{
        db::{
            init_data, init_schemas,
            schemas::wort_review::{InputWortReview, SchemaWortReview, SnapshotWortReview},
            wort::WortRepo,
            wort_review::WortReviewRepo,
        },
        helpers::{error_handler::DbError, time::datetime_2_string},
        test_utils::scenarios::{scenario_wort, scenario_wort_review},
    };

    use rusqlite::Connection;

    fn assert_iter(res: &[SchemaWortReview], data: &[InputWortReview]) {
        assert_eq!(res.len(), data.len());

        for (i, satz) in data.iter().enumerate() {
            assert!(res[i].id > 0);
            assert_eq!(res[i].wort_id, satz.wort_id);
            assert_eq!(res[i].direction, satz.direction.as_str());
            assert_eq!(res[i].interval, satz.interval);
            assert_eq!(res[i].ease_factor, satz.ease_factor);
            assert_eq!(res[i].repetitions, satz.repetitions);
            assert_eq!(res[i].last_review, datetime_2_string(satz.last_review));
            assert_eq!(res[i].next_review, datetime_2_string(satz.next_review));
            assert!(res[i].deleted_at.is_none());
        }
    }

    mod bulk_upsert {
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

            let data = scenario_wort_review().initial;
            let res = WortReviewRepo::bulk_upsert(&mut conn, &[])?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotWortReview> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortReview::bulk_upsert] - empty", snapshot);
            Ok(())
        }

        #[test]
        fn insert() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_review().initial;
            let res = WortReviewRepo::bulk_upsert(&mut conn, &data)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotWortReview> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortReview::bulk_upsert] - insert", snapshot);

            Ok(())
        }

        #[test]
        fn update() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_review().initial;
            WortReviewRepo::bulk_upsert(&mut conn, &data)?;

            let data = scenario_wort_review().update;
            let res = WortReviewRepo::bulk_upsert(&mut conn, &data)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotWortReview> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortReview::bulk_upsert] - update", snapshot);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let data = scenario_wort_review().initial;
            let err = WortReviewRepo::bulk_upsert(&mut conn, &data).unwrap_err();

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
            WortRepo::bulk_insert(&mut conn, &data)?;

            let data = scenario_wort_review().initial;
            WortReviewRepo::bulk_upsert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let res = WortReviewRepo::fetch_by_wort_id(&mut conn, &[])?;

            assert_iter(&res, &[]);

            let snapshot: Vec<SnapshotWortReview> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortReview::fetch_by_wort_id] - empty", snapshot);
            Ok(())
        }

        #[test]
        fn data() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_review().initial;
            let ids_fetched: Vec<i32> = data.iter().map(|d| d.wort_id).collect();
            let res = WortReviewRepo::fetch_by_wort_id(&mut conn, &ids_fetched)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotWortReview> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortReview::fetch_by_wort_id] - data", snapshot);
            Ok(())
        }

        #[test]
        fn not_exists() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let res = WortReviewRepo::fetch_by_wort_id(&mut conn, &[-1])?;

            assert_iter(&res, &[]);

            let snapshot: Vec<SnapshotWortReview> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortReview::fetch_by_wort_id] - empty", snapshot);
            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = WortReviewRepo::fetch_by_wort_id(&mut conn, &[1]).unwrap_err();

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

    mod fetch_new_wort_id_4_review {
        use crate::db::schemas::wort_review::EnumReviewDirection;

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
        fn data_es() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_review().initial;

            let data_es: Vec<i32> = data
                .iter()
                .filter(|f| f.direction == EnumReviewDirection::ES2DE)
                .map(|d| d.wort_id)
                .collect();

            let res =
                WortReviewRepo::fetch_new_wort_id_4_review(&mut conn, EnumReviewDirection::ES2DE)?;
            assert_eq!(res, data_es);

            insta::assert_debug_snapshot!(
                "[WortReview::fetch_new_wort_id_4_review] - data_es_before",
                res
            );

            WortReviewRepo::bulk_upsert(&mut conn, &data)?;

            let res =
                WortReviewRepo::fetch_new_wort_id_4_review(&mut conn, EnumReviewDirection::ES2DE)?;
            assert_eq!(res, Vec::<i32>::new());

            insta::assert_debug_snapshot!(
                "[WortReview::fetch_new_wort_id_4_review] - data_es_after",
                res
            );

            Ok(())
        }

        #[test]
        fn data_de() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_review().initial;

            let data_de: Vec<i32> = data
                .iter()
                .filter(|f| f.direction == EnumReviewDirection::DE2ES)
                .map(|d| d.wort_id)
                .collect();

            let res =
                WortReviewRepo::fetch_new_wort_id_4_review(&mut conn, EnumReviewDirection::DE2ES)?;
            assert_eq!(res, data_de);

            insta::assert_debug_snapshot!(
                "[WortReview::fetch_new_wort_id_4_review] - data_de_before",
                res
            );

            WortReviewRepo::bulk_upsert(&mut conn, &data)?;

            let res =
                WortReviewRepo::fetch_new_wort_id_4_review(&mut conn, EnumReviewDirection::DE2ES)?;
            assert_eq!(res, Vec::<i32>::new());

            insta::assert_debug_snapshot!(
                "[WortReview::fetch_new_wort_id_4_review] - data_de_after",
                res
            );

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err =
                WortReviewRepo::fetch_new_wort_id_4_review(&mut conn, EnumReviewDirection::ES2DE)
                    .unwrap_err();

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

    mod fetch_review_wort_id_by_day {
        use chrono::{DateTime, Duration, Utc};

        use crate::db::schemas::wort_review::EnumReviewDirection;

        use super::*;

        fn fetch_all(conn: &mut Connection, date: DateTime<Utc>) -> Result<Vec<i32>, DbError> {
            let res_1 = WortReviewRepo::fetch_review_wort_id_by_day(
                conn,
                date,
                EnumReviewDirection::ES2DE,
            )?;
            let res_2 = WortReviewRepo::fetch_review_wort_id_by_day(
                conn,
                date,
                EnumReviewDirection::DE2ES,
            )?;
            let mut res: Vec<i32> = res_1;
            res.extend(res_2);
            res.sort_unstable();

            Ok(res)
        }

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            let data = scenario_wort().initial;
            WortRepo::bulk_insert(&mut conn, &data)?;

            let today = Utc::now();
            let data = scenario_wort_review().initial;

            // We modify the next_review value, to make some exercise for fetch
            let data: Vec<_> = data
                .into_iter()
                .map(|d| InputWortReview {
                    next_review: today,
                    ..d
                })
                .collect();
            WortReviewRepo::bulk_upsert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn all() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let today = Utc::now();
            let data = scenario_wort_review().initial;

            // We modify the next_review value, to make some exercise for fetch
            let data: Vec<i32> = data.into_iter().map(|d| d.wort_id).collect();

            let res = fetch_all(&mut conn, today)?;
            assert_eq!(res, data);

            insta::assert_debug_snapshot!("[WortReview::fetch_review_wort_id_by_day] - all", res);

            Ok(())
        }

        /// The objective of this test is to change next_review for ES2DE, and fetch to see if only
        /// the other words are fetched
        #[test]
        fn some_data() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let today = Utc::now();
            let tomorrow = today + Duration::days(1);

            let data = scenario_wort_review().initial;

            // Update words of ES2DE to review tomorrow
            let data_updated: Vec<_> = data
                .iter()
                .filter(|f| f.direction == EnumReviewDirection::ES2DE)
                .cloned()
                .map(|d| InputWortReview {
                    next_review: tomorrow,
                    ..d
                })
                .collect();

            WortReviewRepo::bulk_upsert(&mut conn, &data_updated)?;

            // Get ids from DE2ES reviews
            let data: Vec<_> = data
                .into_iter()
                .filter(|f| f.direction == EnumReviewDirection::DE2ES)
                .map(|d| d.wort_id)
                .collect();

            let res = fetch_all(&mut conn, today)?;
            assert_eq!(res, data);

            insta::assert_debug_snapshot!(
                "[WortReview::fetch_review_wort_id_by_day] - some_data",
                res
            );

            Ok(())
        }

        // It updated all the next_review for tomorrow, so the query should be empty
        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let today = Utc::now();
            let tomorrow = today + Duration::days(1);

            let data = scenario_wort_review().initial;

            // Update all to review tomorrow
            let data_updated: Vec<_> = data
                .into_iter()
                .map(|d| InputWortReview {
                    next_review: tomorrow,
                    ..d
                })
                .collect();

            WortReviewRepo::bulk_upsert(&mut conn, &data_updated)?;

            let res = fetch_all(&mut conn, today)?;
            assert_eq!(res, Vec::<i32>::new());

            insta::assert_debug_snapshot!("[WortReview::fetch_review_wort_id_by_day] - empty", res);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();
            let today = Utc::now();

            let err = WortReviewRepo::fetch_review_wort_id_by_day(
                &mut conn,
                today,
                EnumReviewDirection::ES2DE,
            )
            .unwrap_err();

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

            let data = scenario_wort().initial;
            WortRepo::bulk_insert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let conn = init_conn()?;

            // The table is empty, so this should be empty
            let res = WortReviewRepo::fetch_all_ids(&conn, 100_000, 0)?;
            assert_eq!(res, Vec::<i32>::new());

            insta::assert_debug_snapshot!("[WortReview::fetch_all] - empty", res);

            Ok(())
        }

        #[test]
        fn offset_logic() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_review().initial;
            WortReviewRepo::bulk_upsert(&mut conn, &data)?;

            let limit = 1;
            let mut last_id = 0;
            let mut vec_out: Vec<i32> = vec![];

            loop {
                let res = WortReviewRepo::fetch_all_ids(&conn, limit, last_id)?;

                if res.is_empty() {
                    break;
                }

                last_id = *res.iter().last().unwrap();
                vec_out.extend(res);
            }

            let data: Vec<_> = data.into_iter().map(|d| d.wort_id).collect();

            assert_eq!(vec_out, data);

            insta::assert_debug_snapshot!("[WortReview::fetch_all] - offset_logic", vec_out);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = WortReviewRepo::fetch_all_ids(&mut conn, 100, 0).unwrap_err();

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

    mod deleted_by_id {
        use crate::db::traits::FromSql;

        use super::*;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            let data = scenario_wort().initial;
            WortRepo::bulk_insert(&mut conn, &data)?;

            let data = scenario_wort_review().initial;
            WortReviewRepo::bulk_upsert(&mut conn, &data)?;

            Ok(conn)
        }

        fn fetch_all_data(conn: &Connection) -> Result<Vec<SchemaWortReview>, DbError> {
            let sql = "
                    SELECT 
                        id,
                        wort_id,
                        direction,
                        interval,
                        ease_factor,
                        repetitions,
                        last_review,
                        next_review,
                        created_at,
                        deleted_at
                    FROM 
                        worte_review wr
                    ORDER BY
                        wr.id ASC;
                ";

            let mut stmt = conn.prepare(sql).map_err(DbError::with_sql(sql))?;
            let raw = stmt
                .query([])
                .map_err(DbError::with_sql(sql))?
                .mapped(SchemaWortReview::from_sql)
                .collect::<Result<Vec<_>, _>>()
                .map_err(DbError::with_sql(sql))?;

            Ok(raw)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let fetch_before = fetch_all_data(&conn)?;

            // The table is empty, so this should be empty
            let res = WortReviewRepo::delete_by_id(&mut conn, &[])?;
            assert_eq!(res, 0);

            let fetch_after = fetch_all_data(&conn)?;

            // Check if it doesn't remove anything from DB
            assert_eq!(fetch_before, fetch_after);

            insta::assert_debug_snapshot!("[WortReview::delete_by_id] - empty", res);

            Ok(())
        }

        #[test]
        fn delete_all() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let fetch_before = fetch_all_data(&conn)?;
            let ids_remove: Vec<i32> = fetch_before.into_iter().map(|d| d.wort_id).collect();

            // The table is empty, so this should be empty
            let res = WortReviewRepo::delete_by_id(&mut conn, &ids_remove)?;
            assert_eq!(res, ids_remove.len());

            let fetch_after = fetch_all_data(&conn)?;

            // Check if it remove all ids
            assert_eq!(fetch_after, []);

            insta::assert_debug_snapshot!("[WortReview::delete_by_id] - delete_all", res);

            Ok(())
        }

        #[test]
        fn delete_one() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let fetch_before = fetch_all_data(&conn)?;
            let id_remove: i32 = fetch_before.iter().map(|d| d.wort_id).next().unwrap();

            // The table is empty, so this should be empty
            let res = WortReviewRepo::delete_by_id(&mut conn, &[id_remove])?;
            assert_eq!(res, 1);

            let fetch_after = fetch_all_data(&conn)?;
            let fetch_before: Vec<_> = fetch_before
                .into_iter()
                .filter(|f| f.wort_id != id_remove)
                .collect();

            // Check if it only remove 1 row
            assert_eq!(fetch_before, fetch_after);

            insta::assert_debug_snapshot!("[WortReview::delete_by_id] - delete_one", res);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = WortReviewRepo::delete_by_id(&mut conn, &[1]).unwrap_err();

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
