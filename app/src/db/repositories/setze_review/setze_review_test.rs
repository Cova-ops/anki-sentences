#[cfg(test)]
mod test_setze_review_repo {
    use crate::{
        db::{
            schemas::{
                init_schemas,
                setze_review::{InputSetzeReview, SchemaSetzeReview, SnapshotSetzeReview},
            },
            seeders::init_data,
            setze::SetzeRepo,
            setze_review::SetzeReviewRepo,
        },
        helpers::{
            error_handler::DbError,
            time::{datetime_2_string, string_2_datetime},
        },
        test_utils::scenarios::{scenario_setze, scenario_setze_audio, scenario_setze_review},
    };

    use rusqlite::Connection;

    fn assert_iter(res: &[SchemaSetzeReview], data: &[InputSetzeReview]) {
        assert_eq!(res.len(), data.len());

        for (i, satz) in data.iter().enumerate() {
            assert_eq!(res[i].satz_id, satz.satz_id);
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

            let data = scenario_setze().initial;
            SetzeRepo::bulk_insert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let res = SetzeReviewRepo::bulk_upsert(&mut conn, &[])?;

            let snapshot: Vec<SnapshotSetzeReview> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[SetzeReview::bulk_upsert] - empty", snapshot);

            Ok(())
        }

        #[test]
        fn insert() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data: Vec<_> = scenario_setze_review().initial;
            let res: Vec<_> = SetzeReviewRepo::bulk_upsert(&mut conn, &data)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotSetzeReview> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[SetzeReview::bulk_upsert] - insert", snapshot);

            Ok(())
        }

        #[test]
        fn update() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data: Vec<_> = scenario_setze_review().initial;
            SetzeReviewRepo::bulk_upsert(&mut conn, &data)?;

            let data: Vec<_> = scenario_setze_review().update;
            let res: Vec<_> = SetzeReviewRepo::bulk_upsert(&mut conn, &data)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotSetzeReview> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[SetzeReview::bulk_upsert] - update", snapshot);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let data = scenario_setze_review().initial;
            let err = SetzeReviewRepo::bulk_upsert(&mut conn, &data).unwrap_err();

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

    mod fetch_by_satz_id {
        use super::*;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            let data = scenario_setze().initial;
            SetzeRepo::bulk_insert(&mut conn, &data)?;

            let data = scenario_setze_review().initial;
            SetzeReviewRepo::bulk_upsert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let conn = init_conn()?;

            let res = SetzeReviewRepo::fetch_by_satz_id(&conn, &[])?;
            assert_eq!(res.len(), 0);

            let res: Vec<SnapshotSetzeReview> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[SetzeReview::fetch_by_satz_id] - empty", res);

            Ok(())
        }

        #[test]
        fn with_data() -> Result<(), DbError> {
            let conn = init_conn()?;

            let res = SetzeReviewRepo::fetch_by_satz_id(&conn, &[1, 2])?;
            let data: Vec<_> = scenario_setze_review().initial.drain(0..2).collect();

            assert_iter(&res, &data);

            let res: Vec<SnapshotSetzeReview> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[SetzeReview::fetch_by_satz_id] - with_data", res);

            Ok(())
        }

        #[test]
        fn not_exists() -> Result<(), DbError> {
            let conn = init_conn()?;

            let res = SetzeReviewRepo::fetch_by_satz_id(&conn, &[-1])?;
            let data: Vec<InputSetzeReview> = vec![];

            assert_iter(&res, &data);

            let res: Vec<SnapshotSetzeReview> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[SetzeReview::fetch_by_satz_id] - not_exists", res);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = SetzeReviewRepo::fetch_by_satz_id(&mut conn, &[1, 2]).unwrap_err();

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

    mod fetch_review_satz_id_by_day {
        use chrono::{Duration, Utc};

        use super::*;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            let data = scenario_setze().initial;
            SetzeRepo::bulk_insert(&mut conn, &data)?;

            let today = Utc::now();

            // We modify the vec to make the test
            let data: Vec<InputSetzeReview> = scenario_setze_review()
                .initial
                .into_iter()
                .map(|d| InputSetzeReview {
                    next_review: today,
                    ..d
                })
                .collect();
            SetzeReviewRepo::bulk_upsert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let conn = init_conn()?;

            // This date should not be on scenario_setze_review
            let yesterday = Utc::now() - Duration::days(1);
            let res = SetzeReviewRepo::fetch_review_satz_id_by_day(&conn, yesterday)?;
            assert_eq!(res.len(), 0);

            insta::assert_debug_snapshot!(
                "[SetzeReview::fetch_review_satz_id_by_day] - empty",
                res
            );

            Ok(())
        }

        #[test]
        fn with_data() -> Result<(), DbError> {
            let conn = init_conn()?;

            let today = Utc::now();
            let data = scenario_setze_review().initial;

            // It should bring all on scenario
            let res: Vec<i32> = SetzeReviewRepo::fetch_review_satz_id_by_day(&conn, today)?;
            assert_eq!(res.len(), data.len());

            insta::assert_debug_snapshot!(
                "[SetzeReview::fetch_review_satz_id_by_day] - with_data",
                res
            );
            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let today = Utc::now();
            let err = SetzeReviewRepo::fetch_review_satz_id_by_day(&mut conn, today).unwrap_err();

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
