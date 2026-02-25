#[cfg(test)]
mod test_setze_repo {
    use crate::{
        db::{
            init_schemas,
            schemas::{
                niveau_liste::EnumNiveauListe,
                setze::{InputSetze, SchemaSetze, SnapshotSetze},
            },
            seeders::init_data,
            setze::SetzeRepo,
        },
        helpers::{error_handler::DbError, time::string_2_datetime},
        test_utils::scenarios::scenario_setze,
    };

    use rusqlite::Connection;

    fn assert_iter(res: &[SchemaSetze], data: &[InputSetze]) {
        assert_eq!(res.len(), data.len());

        for (i, satz) in data.iter().enumerate() {
            assert_eq!(res[i].setze_spanisch, satz.setze_spanisch);
            assert_eq!(res[i].setze_deutsch, satz.setze_deutsch);
            assert_eq!(res[i].niveau_id, satz.niveau.id());
            assert_eq!(res[i].thema, satz.thema);
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

            let res = SetzeRepo::bulk_insert(&mut conn, &[])?;

            let snapshot: Vec<SnapshotSetze> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[Setze::bulk_insert] - empty", snapshot);

            Ok(())
        }

        #[test]
        fn after_insert() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let data = scenario_setze().initial;
            let res = SetzeRepo::bulk_insert(&mut conn, &data)?;

            // A couple of hard asserts so we don't rely only on snapshots
            assert!(res[0].id > 0);
            assert!(res[1].id > 0);
            assert_ne!(res[0].id, res[1].id);

            assert_iter(&res, &data);

            let snapshot: Vec<SnapshotSetze> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[Setze::bulk_insert] - after_insert", snapshot);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let data = vec![InputSetze {
                setze_spanisch: "x".into(),
                setze_deutsch: "y".into(),
                niveau: EnumNiveauListe::A2,
                thema: "z".into(),
            }];

            let err = SetzeRepo::bulk_insert(&mut conn, &data).unwrap_err();

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

            let data = scenario_setze().initial;
            SetzeRepo::bulk_insert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn empty() -> Result<(), DbError> {
            let conn = init_conn()?;

            let data: Vec<_> = vec![];
            let res = SetzeRepo::fetch_by_id(&conn, &data)?;
            assert_eq!(res, []);

            let res: Vec<SnapshotSetze> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[Setze::fetch_by_id] - empty", res);

            Ok(())
        }

        #[test]
        fn with_data() -> Result<(), DbError> {
            let conn = init_conn()?;

            let data: Vec<_> = scenario_setze().initial.drain(0..2).collect();
            let res = SetzeRepo::fetch_by_id(&conn, &[1, 2])?;

            assert_iter(&res, &data);

            let res: Vec<SnapshotSetze> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[Setze::fetch_by_id] - with_data", res);

            Ok(())
        }

        #[test]
        fn not_exists() -> Result<(), DbError> {
            let conn = init_conn()?;

            let res = SetzeRepo::fetch_by_id(&conn, &[-1])?;
            assert_eq!(res.len(), 0);

            let res: Vec<SnapshotSetze> = res.into_iter().map(Into::into).collect();
            insta::assert_debug_snapshot!("[Setze::fetch_by_id] - not_exists", res);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = SetzeRepo::fetch_by_id(&mut conn, &[1, 2]).unwrap_err();

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

            let data = scenario_setze().initial;
            SetzeRepo::bulk_insert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn with_data() -> Result<(), DbError> {
            let conn = init_conn()?;

            let data = scenario_setze().initial.into_iter().next().unwrap();
            let res = SetzeRepo::fetch_one(&conn, 1)?.unwrap();

            assert_iter(&[res.clone()], &[data]);

            let res: SnapshotSetze = res.into();
            insta::assert_debug_snapshot!("[Setze::fetch_one] - with_data", res);

            Ok(())
        }

        #[test]
        fn not_exists() -> Result<(), DbError> {
            let conn = init_conn()?;

            let res = SetzeRepo::fetch_one(&conn, -1)?;
            assert!(res.is_none());

            insta::assert_debug_snapshot!("[Setze::fetch_one] - not_exists", res);

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = SetzeRepo::fetch_one(&mut conn, 1).unwrap_err();

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

    mod fetch_id_neue_sentences {
        use crate::db::{schemas::setze_review::InputSetzeReview, setze_review::SetzeReviewRepo};

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
        fn valid_modified() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let res = SetzeRepo::fetch_id_neue_sentences(&conn)?;

            assert_eq!(res.len(), 2);
            assert_eq!(res, [1, 2]);

            insta::assert_debug_snapshot!(
                "[Setze::fetch_id_neue_sentences] - valid_modified (1)",
                res
            );

            SetzeReviewRepo::bulk_upsert(
                &mut conn,
                &[InputSetzeReview {
                    satz_id: 1,
                    repetitions: 1,
                    ease_factor: 2.0,
                    interval: 1,
                    last_review: string_2_datetime("2025-01-10 12:00:00").unwrap(),
                    next_review: string_2_datetime("2025-01-10 12:00:00").unwrap(),
                }],
            )?;

            let res = SetzeRepo::fetch_id_neue_sentences(&conn)?;

            assert_eq!(res.len(), 1);
            assert_eq!(res, [2]);

            insta::assert_debug_snapshot!(
                "[Setze::fetch_id_neue_sentences] - valid_modified (2)",
                res
            );

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = SetzeRepo::fetch_id_neue_sentences(&mut conn).unwrap_err();

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

    mod fetch_id_without_audio {
        use super::*;

        use std::path::PathBuf;

        use crate::{
            db::{schemas::setze_audio::InputSetzeAudio, setze_audio::SetzeAudioRepo},
            services::tts::eleven_labs::EnumVoiceIDElevenLabs,
        };

        fn init_conn() -> Result<Connection, DbError> {
            let mut conn = Connection::open_in_memory()?;
            init_schemas(&mut conn)?;
            init_data(&mut conn)?;

            let data = scenario_setze().initial;
            SetzeRepo::bulk_insert(&mut conn, &data)?;

            Ok(conn)
        }

        #[test]
        fn valid_modified() -> Result<(), DbError> {
            let mut conn = init_conn()?;

            let res = SetzeRepo::fetch_id_without_audio(&conn)?;
            assert_eq!(res.len(), 2);
            assert_eq!(res, [1, 2]);

            insta::assert_debug_snapshot!(
                "[Setze::fetch_setze_without_audio] - valid_modified (1)",
                res
            );

            SetzeAudioRepo::bulk_upsert(
                &mut conn,
                &[InputSetzeAudio {
                    satz_id: 1,
                    voice: EnumVoiceIDElevenLabs::GermanMan,
                    file_path: PathBuf::from("abc"),
                }],
            )?;

            let res = SetzeRepo::fetch_id_without_audio(&conn)?;
            assert_eq!(res.len(), 1);
            assert_eq!(res, [2]);

            insta::assert_debug_snapshot!(
                "[Setze::fetch_id_neue_sentences] - valid_modified (2)",
                res
            );

            Ok(())
        }

        #[test]
        fn error() -> Result<(), DbError> {
            // Use a raw in-memory conn without your schema to force a prepare/query failure.
            // This verifies DbError::with_sql(sql) is attaching the SQL.
            let mut conn = Connection::open_in_memory().unwrap();

            let err = SetzeRepo::fetch_id_without_audio(&mut conn).unwrap_err();

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
