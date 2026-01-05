use color_eyre::eyre::{Context, Result};

use crate::{
    db::{get_conn, schemas::worte_audio::NewWorteAudioSchema, worte_audio::WorteAudioRepo},
    helpers::{audios::ManageAudios, toml::AppConfig},
    services::tts::{self, eleven_labs::LanguageVoice},
    utils,
};

fn process_audio(
    audio_name: Option<String>,
    text: &str,
    wort_id: i32,
    manage_audios: &ManageAudios,
    lang: LanguageVoice,
) -> Result<String> {
    let res = if let Some(name) = audio_name {
        name
    } else {
        let audio_bytes: Vec<u8> = tts::eleven_labs::generate_tts(text, lang)?;
        let audio_path = manage_audios.save_audio_worte(audio_bytes, wort_id, lang)?;
        let audio_name = utils::path::get_filename_from_path(&audio_path)?;

        audio_name
    };

    Ok(res)
}

fn try_audio(
    audio_name: Option<String>,
    text: &str,
    wort_id: i32,
    manage_audios: &ManageAudios,
    lang: LanguageVoice,
) -> Option<String> {
    match process_audio(audio_name, text, wort_id, manage_audios, lang)
        .wrap_err_with(|| format!("processing wort_id={wort_id}, lang={lang:?}, text={text}"))
    {
        Ok(name) => Some(name),
        Err(err) => {
            eprintln!("{:#?}", err);
            None
        }
    }
}

pub fn run(config: &AppConfig) -> Result<()> {
    let mut conn = get_conn(config.get_database_path()?)?;

    let worte_without_audio = WorteAudioRepo::fetch_worte_without_audio(&conn)?;
    let len_vec = worte_without_audio.len();

    let manage_audios = ManageAudios::new(
        config.get_path_audios_worte()?,
        config.get_path_audios_setze()?,
    );

    for (i, wort) in worte_without_audio.into_iter().enumerate() {
        let audio_name_es = try_audio(
            wort.audio_name_es,
            &wort.wort_es,
            wort.id,
            &manage_audios,
            LanguageVoice::Spanisch,
        );

        let audio_name_de = try_audio(
            wort.audio_name_de,
            &wort.wort_de,
            wort.id,
            &manage_audios,
            LanguageVoice::Deutsch,
        );

        if audio_name_es.is_none() && audio_name_de.is_none() {
            continue;
        }

        WorteAudioRepo::bulk_insert(
            &mut conn,
            &[NewWorteAudioSchema {
                wort_id: wort.id,
                audio_name_es,
                audio_name_de,
            }],
        )?;

        println!("Processed wort_id={} ({}/{}).", wort.id, i + 1, len_vec);
    }

    println!();
    println!("Download of audios is completed :).");
    println!();

    Ok(())
}
