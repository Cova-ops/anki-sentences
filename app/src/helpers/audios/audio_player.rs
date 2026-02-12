use std::{fs::File, io::BufReader};

use color_eyre::eyre::Result;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};

use crate::{helpers::audios::ManageAudios, services::tts::eleven_labs::LanguageVoice};

pub struct AudioPlayer {
    stream: OutputStream,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let stream =
            OutputStreamBuilder::open_default_stream().expect("Error al inicializar las bocinas");
        Self { stream }
    }

    pub fn play(&self, file: File) -> Result<()> {
        let sink = Sink::connect_new(self.stream.mixer());

        // Abre el archivo mp3
        let source = Decoder::new(BufReader::new(file))?;

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
        gender: &Option<WorteGenderSchema>,
        lang: LanguageVoice,
    ) -> Result<()> {
        if id_wort.is_none() {
            return Ok(());
        }

        let id_wort = id_wort.unwrap();
        if lang == LanguageVoice::Deutsch {
            if let Some(gender) = gender.as_ref() {
                let path_artikel = manage_audio
                    .get_audio_artikel(GenderGermanListe::try_from(gender.artikel.as_str())?);

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
