use std::{fs::File, io::BufReader};

use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};

use crate::{
    db::schemas::wort_gender::EnumWortGender,
    helpers::{
        audios::ManageAudios,
        error_handler::{AppError, AppErrorKind, AudioError},
    },
    services::tts::eleven_labs::LanguageVoice,
};

pub struct AudioPlayer {
    stream: OutputStream,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let stream =
            OutputStreamBuilder::open_default_stream().expect("Error al inicializar las bocinas");
        Self { stream }
    }

    pub fn play(&self, file: File) -> Result<(), AppError> {
        let sink = Sink::connect_new(self.stream.mixer());

        // Abre el archivo mp3
        let source = Decoder::new(BufReader::new(file)).map_err(|e| AppError {
            kind: AppErrorKind::Audio(AudioError::Decoder(e)),
            context: vec![],
        })?;

        // Lo mandas al sink
        sink.append(source);

        // Esperas a que termine de reproducir
        sink.sleep_until_end();

        Ok(())
    }

    pub fn play_from_path(
        &self,
        manage_audio: &ManageAudios,
        id_wort: Option<&i32>,
        gender: &Option<EnumWortGender>,
        lang: LanguageVoice,
    ) -> Result<(), AppError> {
        if id_wort.is_none() {
            return Ok(());
        }

        let id_wort = id_wort.unwrap();
        if lang == LanguageVoice::Deutsch {
            if let Some(gender) = gender.as_ref() {
                let path_artikel = manage_audio.get_audio_artikel(gender);

                if let Ok(path) = path_artikel {
                    self.play(path)?;
                }
            }
        }

        let path_word = manage_audio.get_audio_worte(*id_wort, lang);
        if let Ok(Some(path)) = path_word {
            self.play(path)?;
        }

        Ok(())
    }
}
