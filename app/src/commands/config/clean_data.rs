use std::{collections::HashSet, fs::remove_file};

use color_eyre::eyre::Result;
use rusqlite::Connection;

use crate::{
    console::cli::TypeFile,
    db::{
        get_conn, worte::WorteRepo, worte_audio::WorteAudioRepo,
        worte_gram_type::WorteGramTypeRepo, worte_review::WorteReviewRepo,
    },
    helpers::{
        audios::{AudioKind, ManageAudios},
        toml::AppConfig,
    },
    services::tts::eleven_labs::LanguageVoice,
};

fn collect_orphans<F>(
    conn: &Connection,
    limit: usize,
    existing: &HashSet<i32>,
    out_remove: &mut HashSet<i32>,
    mut fetch_ids: F,
) -> Result<()>
where
    F: FnMut(&Connection, usize, i32) -> Result<Vec<i32>>,
{
    let mut last_id = 0;
    loop {
        let vec_ids = fetch_ids(conn, limit, last_id)?;

        if vec_ids.is_empty() {
            break;
        }

        last_id = *vec_ids.last().unwrap();
        for id in vec_ids {
            if !existing.contains(&id) {
                out_remove.insert(id);
            }
        }
    }

    Ok(())
}

pub fn run(config: &AppConfig) -> Result<()> {
    let mut conn = get_conn(config.get_database_path()?)?;

    let limit = 1000;
    let mut last_id = 0;
    let mut hash_ids: HashSet<i32> = HashSet::new();

    loop {
        let vec_ids = WorteRepo::fetch_all_ids(&conn, limit, last_id)?;

        if vec_ids.is_empty() {
            break;
        }

        last_id = *vec_ids.last().unwrap();
        hash_ids.extend(vec_ids);
    }

    let mut hash_ids_remove: HashSet<i32> = HashSet::new();

    collect_orphans(
        &conn,
        limit,
        &hash_ids,
        &mut hash_ids_remove,
        WorteAudioRepo::fetch_all_ids,
    )?;

    collect_orphans(
        &conn,
        limit,
        &hash_ids,
        &mut hash_ids_remove,
        WorteReviewRepo::fetch_all_ids,
    )?;

    collect_orphans(
        &conn,
        limit,
        &hash_ids,
        &mut hash_ids_remove,
        WorteGramTypeRepo::fetch_all_ids,
    )?;

    let ids_remove: Vec<i32> = hash_ids_remove.into_iter().collect();

    let rows_affected = WorteAudioRepo::delete_by_id(&conn, &ids_remove)?;
    println!("Rows affected on table worte_audio: {}", rows_affected);

    let rows_affected = WorteReviewRepo::delete_by_id(&conn, &ids_remove)?;
    println!("Rows affected on table worte_review: {}", rows_affected);

    let rows_affected = WorteGramTypeRepo::delete_by_id(&mut conn, &ids_remove)?;
    println!("Rows affected on table worte_gram_type: {}", rows_affected);

    // TODO: tambien valdiar los audios locales

    let manage_audios = ManageAudios::new(
        config.get_path_audios_worte()?,
        config.get_path_audios_setze()?,
    );
    let (mut hash_mp3_worte, _) = manage_audios.get_all_ids_files()?;

    // remove audios that are on DB
    for audio in hash_ids.iter() {
        hash_mp3_worte.remove(audio);
    }

    // the ids that are the remaining on hash_mp3_worte, are audios that need to be eliminated
    manage_audios.remove_audios(hash_mp3_worte, AudioKind::Wort)?;

    Ok(())
}
