#[cfg(test)]
mod test_wort_gender_repo {
    use crate::{
        db::{
            schemas::{
                init_schemas,
                wort_gender::{InputWortGender, SchemaWortGender, SnapshotWortGender},
            },
            wort_gender::WortGenderRepo,
        },
        helpers::error_handler::DbError,
        test_utils::scenarios::scenario_wort_gender,
    };
    use rusqlite::Connection;

    fn assert_iter(res: &[SchemaWortGender], data: &[InputWortGender]) {
        assert_eq!(res.len(), data.len());

        for (i, satz) in data.iter().enumerate() {
            assert_eq!(res[i].gender, satz.gender.gender());
            assert!(res[i].deleted_at.is_none());
        }
    }

    mod bulk_upsert {
        use super::*;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let res = WortGenderRepo::bulk_upsert(&mut conn, &[])?;

            assert_iter(&res, &[]);

            let snapshot: Vec<SnapshotWortGender> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortGender::bulk_upsert] - empty", snapshot);

            Ok(())
        }

        #[test]
        fn insert() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_gender().initial;
            let res = WortGenderRepo::bulk_upsert(&mut conn, &data)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotWortGender> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortGender::bulk_upsert] - insert", snapshot);

            Ok(())
        }

        #[test]
        fn update() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_gender().initial;
            WortGenderRepo::bulk_upsert(&mut conn, &data)?;

            let data = scenario_wort_gender().update;
            let res = WortGenderRepo::bulk_upsert(&mut conn, &data)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotWortGender> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortGender::bulk_upsert] - upsert", snapshot);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let data = scenario_wort_gender().initial;
            let err = WortGenderRepo::bulk_upsert(&mut conn, &data).unwrap_err();

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
