#[cfg(test)]
mod test_wort_audio_repo {
    use rusqlite::Connection;

    use crate::{
        db::{
            init_schemas,
            schemas::wort_audio::{InputWortAudio, SchemaWortAudio, SnapshotWortAudio},
            seeders::init_data,
            wort_audio::WortAudioRepo,
        },
        helpers::{error_handler::DbError, time::string_2_datetime},
        test_utils::scenarios::scenario_wort_audio,
    };

    fn assert_iter(res: &[SchemaWortAudio], data: &[InputWortAudio]) {
        assert_eq!(res.len(), data.len());

        for (i, satz) in data.iter().enumerate() {
            assert!(res[i].id > 0);

            assert_eq!(res[i].wort_id, satz.wort_id);
            assert_eq!(res[i].audio_name_es, satz.audio_name_es);
            assert_eq!(res[i].audio_name_de, satz.audio_name_de);

            assert!(string_2_datetime(res[i].created_at).is_ok());
            assert!(res[i].deleted_at.is_none());
        }
    }

    mod bulk_upsert {
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

            let res = WortAudioRepo::bulk_upsert(&mut conn, &[])?;

            assert_iter(&res, &[]);

            let snapshot: Vec<SnapshotWortAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortAudioRepo::bulk_upsert] - empty", snapshot);

            Ok(())
        }

        #[test]
        fn insert() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_audio().initial;
            let res = WortAudioRepo::bulk_upsert(&mut conn, &data)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotWortAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortAudioRepo::bulk_upsert] - insert", snapshot);

            Ok(())
        }

        #[test]
        fn update() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_audio().initial;
            WortAudioRepo::bulk_upsert(&mut conn, &data)?;

            let data = scenario_wort_audio().update;
            let res = WortAudioRepo::bulk_upsert(&mut conn, &data)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotWortAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortAudioRepo::bulk_upsert] - update", snapshot);
            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let data = scenario_wort_audio().initial;
            let err = WortAudioRepo::bulk_upsert(&mut conn, &data).unwrap_err();

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

            let data = scenario_wort_audio().initial;
            WortAudioRepo::bulk_upsert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let res = WortAudioRepo::fetch_by_id(&mut conn, &[])?;

            assert_iter(&res, &[]);

            let snapshot: Vec<SnapshotWortAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortAudioRepo::fetch_by_id] - empty", snapshot);

            Ok(())
        }

        #[test]
        fn data() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_audio().initial;
            let res = WortAudioRepo::fetch_by_id(&mut conn, &[1, 2])?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotWortAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortAudioRepo::fetch_by_id] - data", snapshot);

            Ok(())
        }

        #[test]
        fn not_exists() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let res = WortAudioRepo::fetch_by_id(&mut conn, &[-1])?;

            assert_iter(&res, &[]);

            let snapshot: Vec<SnapshotWortAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortAudioRepo::fetch_by_id] - not_exists", snapshot);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = WortAudioRepo::fetch_by_id(&mut conn, &[1, 2]).unwrap_err();

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

            let data = scenario_wort_audio().initial;
            WortAudioRepo::bulk_upsert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn all_data() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_audio().initial;
            let res = WortAudioRepo::fetch_all_ids(&mut conn, 100_000, 0)?;

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotWortAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortAudioRepo::fetch_all_ids] - all_data", snapshot);

            Ok(())
        }

        #[test]
        fn offset_logic() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_audio().initial;

            let limit = 1;
            let mut last_id = 0;
            let mut vec_out: Vec<i32> = vec![];
            loop {
                let mut res = WortAudioRepo::fetch_all_ids(&mut conn, limit, last_id)?;
                if res.is_empty() {
                    break;
                }

                last_id = res.last().unwrap().wort_id;
                vec_out.append(&mut res);
            }

            assert_iter(&vec_out, &data);

            let snapshot: Vec<SnapshotWortAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!(
                "[WortAudioRepo::fetch_all_ids] - offset_logic",
                snapshot
            );

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = WortAudioRepo::fetch_all_ids(&mut conn, 100, 0).unwrap_err();

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

    mod fetch_worte_without_audio {
        use super::*;
        use crate::{
            db::views::wort_audio_missing::SnapshotWortAudioMissing,
            test_utils::scenarios::scenario_wort,
        };

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            Ok(conn)
        }

        #[test]
        fn without_insert() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort().initial;
            let res = WortAudioRepo::fetch_worte_without_audio(&mut conn)?;

            assert_eq!(res.len(), data.len());
            for (i, wort) in data.iter().enumerate() {
                assert_eq!(res[i].wort_es, wort.worte_es);
                assert_eq!(res[i].wort_de, wort.worte_de);
                assert_eq!(res[i].audio_name_es, None);
                assert_eq!(res[i].audio_name_de, None);
            }

            let snapshot: Vec<SnapshotWortAudioMissing> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!(
                "[WortAudioRepo::fetch_worte_without_audio] - without_insert",
                snapshot
            );

            Ok(())
        }

        #[test]
        fn with_insert() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_wort_audio().initial;
            WortAudioRepo::bulk_upsert(&mut conn, &data)?;

            let res = WortAudioRepo::fetch_worte_without_audio(&mut conn)?;
            let data: Vec<_> = data
                .into_iter()
                .filter(|f| f.audio_name_es.is_none() || f.audio_name_de.is_none())
                .collect();

            assert_eq!(res.len(), data.len());
            for (i, wort) in data.iter().enumerate() {
                assert_eq!(res[i].wort_es, wort.worte_es);
                assert_eq!(res[i].wort_de, wort.worte_de);
                assert_eq!(res[i].audio_name_es, wort.audio_name_es);
                assert_eq!(res[i].audio_name_de, wort.audio_name_de);
            }

            let snapshot: Vec<SnapshotWortAudioMissing> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!(
                "[WortAudioRepo::fetch_worte_without_audio] - with_insert",
                snapshot
            );

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = WortAudioRepo::fetch_worte_without_audio(&mut conn).unwrap_err();

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

    mod delete_by_id {
        use crate::db::traits::FromSql;

        use super::*;

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            let data = scenario_wort_audio().initial;
            WortAudioRepo::bulk_upsert(&mut conn, &data)?;

            Ok(conn)
        }

        fn fetch_all_data(conn: &Connection) -> Result<Vec<SchemaWortAudio>, DbError> {
            let sql = "
                SELECT
                    wort_id,
                    audio_name_es,
                    audio_name_de,
                    created_at,
                    deleted_at
                FROM worte_audio
                ORDER BY wort_id;
            ";

            let mut stmt = conn.prepare(sql).map_err(DbError::with_sql(sql))?;
            let raw = stmt
                .query([])
                .map_err(DbError::with_sql(sql))?
                .mapped(SchemaWortAudio::from_sql)
                .collect::<Result<Vec<_>, _>>()
                .map_err(DbError::with_sql(sql))?;

            Ok(raw)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            // Check if it doesn't remove anything from DB
            let fetch_before = fetch_all_data(&conn)?;

            let res = WortAudioRepo::delete_by_id(&mut conn, &[])?;
            assert_eq!(res, 0);

            // Check if it doesn't remove anything from DB
            let fetch_after = fetch_all_data(&conn)?;

            assert_eq!(fetch_before, fetch_after);

            let snapshot: Vec<SnapshotWortAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortAudioRepo::delete_by_id] - empty", snapshot);

            Ok(())
        }

        #[test]
        fn delete_all() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data: Vec<_> = scenario_wort_audio().initial;
            let mut ids_remove: Vec<i32> = data.into_iter().map(|d| d.wort_id).collect();

            // remove ids duplicated if exists
            ids_remove.sort_unstable();
            ids_remove.dedup();

            let res = WortAudioRepo::delete_by_id(&mut conn, &ids_remove)?;
            assert_eq!(res, ids_remove.len());

            // Check if it remove all from the DB
            let fetch_after = fetch_all_data(&conn)?;
            assert_eq!(fetch_after, vec![]);

            let snapshot: Vec<SnapshotWortAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortAudioRepo::delete_by_id] - delete_all", snapshot);

            Ok(())
        }

        #[test]
        fn delete_one() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let fetch_before = fetch_all_data(&conn)?;

            let data: Vec<_> = scenario_wort_audio().initial;
            let id_remove: i32 = data.into_iter().map(|d| d.wort_id).next().unwrap();

            let res = WortAudioRepo::delete_by_id(&mut conn, &[id_remove])?;
            assert_eq!(res, 1);

            // Check if it remove all from the DB
            let fetch_after = fetch_all_data(&conn)?;

            // Using fetch_before remove the wor that should be removed on DB, and compare the rest
            // data
            let fetch_before: Vec<_> = fetch_before
                .into_iter()
                .filter(|f| f.wort_id != id_remove)
                .collect();

            assert_eq!(fetch_before, fetch_after);

            let snapshot: Vec<SnapshotWortAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[WortAudioRepo::delete_by_id] - delete_one", snapshot);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = WortAudioRepo::delete_by_id(&mut conn, &[]).unwrap_err();

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
