#[cfg(test)]
mod test_setze_audio_repo {
    use crate::{
        db::{
            schemas::{
                init_schemas,
                setze_audio::{InputSetzeAudio, SnapshotSetzeAudio},
            },
            seeders::init_data,
            setze_audio::SetzeAudioRepo,
        },
        helpers::error_handler::DbError,
        services::tts::eleven_labs::EnumVoiceIDElevenLabs,
        test_utils::scenarios::scenario_setze_audio,
    };

    use rusqlite::Connection;
    use std::path::PathBuf;

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

            let res = SetzeAudioRepo::bulk_upsert(&mut conn, &[])?;

            let snapshot: Vec<SnapshotSetzeAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[SetzeAudio::bulk_insert] - empty", snapshot);

            Ok(())
        }

        #[test]
        fn insert() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data: Vec<_> = scenario_setze_audio().initial;
            let res: Vec<_> = SetzeAudioRepo::bulk_upsert(&tx, &data)?;

            // A couple of hard asserts so we don't rely only on snapshots
            assert_eq!(res.len(), data.len());

            for (i, satz) in data.iter().enumerate() {
                assert_eq!(res[i].satz_id, satz.satz_id);
                assert_eq!(res[i].file_path, satz.file_path.to_string_lossy());
                assert_eq!(res[i].voice_id, satz.voice.get_key());
                assert!(res[i].deleted_at.is_none());
            }

            let snapshot: Vec<SnapshotSetzeAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[SetzeAudio::bulk_upsert] - insert", snapshot);

            Ok(())
        }

        #[test]
        fn update() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data: Vec<_> = scenario_setze_audio().initial;
            SetzeAudioRepo::bulk_upsert(&mut conn, &data)?;

            let data: Vec<_> = scenario_setze_audio().update;
            let res: Vec<_> = SetzeAudioRepo::bulk_upsert(&mut conn, &data)?;

            // A couple of hard asserts so we don't rely only on snapshots
            assert_eq!(res.len(), data.len());

            for (i, satz) in data.iter().enumerate() {
                assert_eq!(res[i].satz_id, satz.satz_id);
                assert_eq!(res[i].file_path, satz.file_path.to_string_lossy());
                assert_eq!(res[i].voice_id, satz.voice.get_key());
                assert!(res[i].deleted_at.is_none());
            }

            let snapshot: Vec<SnapshotSetzeAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[SetzeAudio::bulk_upsert] - update", snapshot);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let data = vec![InputSetzeAudio {
                satz_id: 1,
                file_path: PathBuf::from("a"),
                voice: EnumVoiceIDElevenLabs::GermanMan,
            }];

            let err = SetzeAudioRepo::bulk_upsert(&mut conn, &data).unwrap_err();

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

            let data = scenario_setze_audio().initial;
            SetzeAudioRepo::bulk_upsert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let conn = init_conn()?;

            let res = SetzeAudioRepo::fetch_by_id(&conn, &[])?;
            assert_eq!(res.len(), 0);

            let res: Vec<SnapshotSetzeAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[SetzeAudio::fetch_by_id] - empty", res);

            Ok(())
        }

        #[test]
        fn with_data() -> Result<(), DbError> {
            let conn = init_conn()?;

            let res = SetzeAudioRepo::fetch_by_id(&conn, &[1, 2])?;
            assert_eq!(res.len(), 2);

            for (i, satz) in scenario_setze_audio().initial.drain(0..2).enumerate() {
                assert_eq!(res[i].satz_id, satz.satz_id);
                assert_eq!(res[i].file_path, satz.file_path.to_string_lossy());
                assert_eq!(res[i].voice_id, satz.voice.get_key());
                assert!(res[i].deleted_at.is_none());
            }

            let res: Vec<SnapshotSetzeAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[SetzeAudio::fetch_by_id] - with_data", res);

            Ok(())
        }

        #[test]
        fn not_exists() -> Result<(), DbError> {
            let conn = init_conn()?;

            let res = SetzeAudioRepo::fetch_by_id(&conn, &[-1])?;
            assert_eq!(res.len(), 0);

            let res: Vec<SnapshotSetzeAudio> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[SetzeAudio::fetch_by_id] - not_exists", res);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = SetzeAudioRepo::fetch_by_id(&mut conn, &[1, 2]).unwrap_err();

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

