use color_eyre::eyre::Result;
use rusqlite::Connection;

use crate::{
    db::{
        schemas::worte_audio::NewWorteAudioSchema, worte::WorteRepo, worte_audio::WorteAudioRepo,
    },
    helpers::audios::ManageAudios,
    services::tts::{self, eleven_labs::LanguageVoice},
};

pub fn menu_5_2_audios_on_worte(conn: &mut Connection) -> Result<()> {
    let worte_without_audio: Vec<_> = WorteRepo::fetch_worte_without_audio(conn)?;
    let len_vec = worte_without_audio.len();

    for (i, wort) in worte_without_audio.iter().enumerate() {
        let audio_bytes: Vec<u8> =
            match tts::eleven_labs::generate_tts(&wort.worte_es, LanguageVoice::Spanisch) {
                Ok(v) => v,
                Err(err) => {
                    println!("Error al generar el TTS de la palabra: {}", wort.worte_es);
                    println!("{:#?}", err);
                    continue;
                }
            };

        let audio_path = match ManageAudios::save_audio_worte(audio_bytes, wort.id) {
            Ok(v) => v,
            Err(err) => {
                println!("Error al guardar el archivo: {}", wort.worte_es);
                println!("{:#?}", err);
                continue;
            }
        };

        WorteAudioRepo::bulk_insert(
            conn,
            &[NewWorteAudioSchema {
                wort_id: wort.id,
                voice_id: "masc_eleven_labs".into(),
                file_path: audio_path,
            }],
        )?;

        println!("Audio procesado {}/{}.", i + 1, len_vec);
    }

    println!();
    println!("Descarga de audios completada :).");
    println!();

    Ok(())
}
