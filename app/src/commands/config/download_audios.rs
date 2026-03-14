use crate::{
    db::{
        get_conn, schemas::wort_audio::InputWortAudio,
        views::wort_audio_missing::ModelWortAudioMissing, wort_audio::WortAudioRepo,
    },
    helpers::{audios::ManageAudios, error_handler::AppError, toml::AppConfig},
    services::tts::{self, language_voice::LanguageVoice},
    utils,
};

fn process_audio(
    audio_name: Option<String>,
    text: &str,
    wort_id: i32,
    manage_audios: &ManageAudios,
    lang: LanguageVoice,
) -> Result<String, AppError> {
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
    match process_audio(audio_name, text, wort_id, manage_audios, lang) {
        Ok(name) => Some(name),
        Err(err) => {
            eprintln!("{:#?}", err);
            None
        }
    }
}

pub fn run(config: &AppConfig) -> Result<(), AppError> {
    let mut conn = get_conn(config.get_database_path()?)?;

    let worte_without_audio = WortAudioRepo::fetch_worte_without_audio(&conn)?;
    let worte_without_audio: Vec<ModelWortAudioMissing> = worte_without_audio
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<_, _>>()?;

    let len_vec = worte_without_audio.len();

    let manage_audios = ManageAudios::new(
        config.get_path_audios_worte()?,
        config.get_path_audios_setze()?,
        config.get_path_audios_artikel()?,
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

        WortAudioRepo::bulk_upsert(
            &mut conn,
            &[InputWortAudio {
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
