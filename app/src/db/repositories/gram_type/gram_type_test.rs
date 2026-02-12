#[cfg(test)]
mod tests_gram_type_repo_bulk_upsert {
    use rusqlite::{Connection, params};

    use crate::{
        db::{
            gram_type::GramTypeRepo,
            schemas::gram_type::{InputGramType, SchemaGramType, SnapshotGramType},
        },
        helpers::error_handler::DbError,
        test_utils::scenarios::scenario_gram_type,
    };

    fn assert_iter(res: &[SchemaGramType], data: &[InputGramType]) {
        assert_eq!(res.len(), data.len());

        for (i, gram) in data.iter().enumerate() {
            assert_eq!(res[i].code, gram.gram.to_code());
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

            let res = GramTypeRepo::bulk_upsert(&mut conn, &[])?;

            assert_iter(&res, &[]);

            let snapshot: Vec<SnapshotGramType> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[GramType::bulk_upsert] - empty", snapshot);

            Ok(())
        }

        #[test]
        fn insert() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_gram_type().initial;
            let res = GramTypeRepo::bulk_upsert(&mut conn, &data)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotGramType> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[GramType::bulk_upsert] - insert", snapshot);

            Ok(())
        }

        #[test]
        fn update() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_gram_type().initial;
            GramTypeRepo::bulk_upsert(&mut conn, &data)?;

            // Force UPDATE via conflict(id): same id, wrong code/name
            {
                conn.execute(
                    r#"
                UPDATE gram_type
                SET code = ?2, name = ?3
                WHERE id = ?1
                "#,
                    params![0i32, "WRONG_CODE", "WRONG_NAME"],
                )?;
            }

            // calling upsert again should correct the row back to enum-derived code/name
            let res = GramTypeRepo::bulk_upsert(&mut conn, &data)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotGramType> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[GramType::bulk_upsert] - update", snapshot);
            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let data = scenario_gram_type().initial;
            let err = GramTypeRepo::bulk_upsert(&mut conn, &data).unwrap_err();

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
