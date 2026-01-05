use crate::db::setup_test_db;

#[cfg(test)]
mod test_worte_audio_repo {

    use crate::db::{
        schemas::worte_audio::{NewWorteAudioSchema as New, WorteAudioSchema as Schema},
        worte_audio::WorteAudioRepo,
    };

    use super::*;

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Snapshot {
        wort_id: i32,
        audio_name_es: Option<String>,
        audio_name_de: Option<String>,
        created_at: String,
        deleted_at: String,
    }

    fn placeholder_dates(data: Vec<Schema>) -> Vec<Snapshot> {
        data.into_iter()
            .map(|d| Snapshot {
                wort_id: d.wort_id,
                audio_name_es: d.audio_name_es,
                audio_name_de: d.audio_name_de,
                created_at: "<created_at>".into(),
                deleted_at: "<deleted_at>".into(),
            })
            .collect()
    }

    mod bulk_insert {
        use std::{thread, time::Duration};

        use color_eyre::eyre::Result;
        use rusqlite::Connection;

        use crate::db::{schemas::worte::NewWorteSchema, seeders::init_data, worte::WorteRepo};

        use super::*;

        fn init_data_local(conn: &mut Connection) -> Result<()> {
            init_data(conn)?;
            let data = vec![
                NewWorteSchema {
                    gram_type: vec![1],
                    gender_id: Some(1),
                    worte_de: "Hund".into(),
                    worte_es: "Perro".into(),
                    plural: Some("Hunde".into()),
                    niveau_id: 1,
                    example_de: "Beispiel".into(),
                    example_es: "Ejemplo".into(),
                    verb_aux: None,
                    trennbar: None,
                    reflexiv: None,
                },
                NewWorteSchema {
                    gram_type: vec![2, 3],
                    gender_id: None,
                    worte_de: "laufen".into(),
                    worte_es: "correr".into(),
                    plural: None,
                    niveau_id: 2,
                    example_de: "Beispiel".into(),
                    example_es: "Ejemplo".into(),
                    verb_aux: Some("sein".into()),
                    trennbar: Some(false),
                    reflexiv: Some(false),
                },
            ];
            WorteRepo::bulk_insert(conn, &data)?;
            Ok(())
        }

        fn run_bulk_insert_update_scenario<F1, F2>(c1: F1, c2: F2)
        where
            F1: FnOnce(&mut Connection) -> Result<Vec<Schema>>,
            F2: FnOnce(&mut Connection) -> Result<Vec<Schema>>,
        {
            let mut conn = setup_test_db().unwrap();
            init_data_local(&mut conn).expect("Error al iniciar datos dummy");

            let res_1 = c1(&mut conn).expect("La inserción no debe fallar");

            assert_eq!(res_1.len(), 2);

            assert_eq!(res_1[0].wort_id, 1);
            assert_eq!(res_1[0].audio_name_es, Some("12345_es.mp3".into()));
            assert_eq!(res_1[0].audio_name_de, Some("12345_de.mp3".into()));

            assert_eq!(res_1[1].wort_id, 2);
            assert_eq!(res_1[1].audio_name_es, Some("abcde_es.mp3".into()));
            assert_eq!(res_1[1].audio_name_de, Some("abcde_de.mp3".into()));

            let res_1 = placeholder_dates(res_1);
            insta::assert_debug_snapshot!(res_1);

            thread::sleep(Duration::from_millis(100));

            let res_2 = c2(&mut conn).expect("La actualización no debe fallar");

            assert_eq!(res_2[0].wort_id, 1);
            assert_eq!(res_2[0].audio_name_es, Some("12345_es.mp4".into()));
            assert_eq!(res_2[0].audio_name_de, Some("12345_de.mp4".into()));

            assert_eq!(res_2[1].wort_id, 2);
            assert_eq!(res_2[1].audio_name_es, Some("abcde_es.mp4".into()));
            assert_eq!(res_2[1].audio_name_de, Some("abcde_de.mp4".into()));

            let res_2 = placeholder_dates(res_2);
            insta::assert_debug_snapshot!(res_2);
        }

        #[test]
        fn test_bulk_insert_and_update() {
            let data_1 = vec![
                New {
                    wort_id: 1,
                    audio_name_es: Some("12345_es.mp3".into()),
                    audio_name_de: Some("12345_de.mp3".into()),
                },
                New {
                    wort_id: 2,
                    audio_name_es: Some("abcde_es.mp3".into()),
                    audio_name_de: Some("abcde_de.mp3".into()),
                },
            ];
            let data_2 = vec![
                New {
                    wort_id: 1,
                    audio_name_es: Some("12345_es.mp4".into()),
                    audio_name_de: Some("12345_de.mp4".into()),
                },
                New {
                    wort_id: 2,
                    audio_name_es: Some("abcde_es.mp4".into()),
                    audio_name_de: Some("abcde_de.mp4".into()),
                },
            ];
            run_bulk_insert_update_scenario(
                |conn| WorteAudioRepo::bulk_insert(conn, &data_1),
                |conn| WorteAudioRepo::bulk_insert(conn, &data_2),
            );
        }

        #[test]
        fn test_bulk_insert_and_update_tx() {
            let data_1 = vec![
                New {
                    wort_id: 1,
                    audio_name_es: Some("12345_es.mp3".into()),
                    audio_name_de: Some("12345_de.mp3".into()),
                },
                New {
                    wort_id: 2,
                    audio_name_es: Some("abcde_es.mp3".into()),
                    audio_name_de: Some("abcde_de.mp3".into()),
                },
            ];
            let data_2 = vec![
                New {
                    wort_id: 1,
                    audio_name_es: Some("12345_es.mp4".into()),
                    audio_name_de: Some("12345_de.mp4".into()),
                },
                New {
                    wort_id: 2,
                    audio_name_es: Some("abcde_es.mp4".into()),
                    audio_name_de: Some("abcde_de.mp4".into()),
                },
            ];
            run_bulk_insert_update_scenario(
                |conn| {
                    let tx = conn.transaction()?;
                    let out = WorteAudioRepo::bulk_insert_tx(&tx, &data_1)?;
                    tx.commit()?;
                    Ok(out)
                },
                |conn| {
                    let tx = conn.transaction()?;
                    let out = WorteAudioRepo::bulk_insert_tx(&tx, &data_2)?;
                    tx.commit()?;
                    Ok(out)
                },
            );
        }
    }
}
