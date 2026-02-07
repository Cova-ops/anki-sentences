use std::path::PathBuf;

use crate::services::tts::eleven_labs::EnumVoiceIDElevenLabs;

#[derive(Debug)]
pub struct SqlSetzeAudio {
    pub satz_id: i32,
    pub file_path: String,
    pub voice_id: String,
}

#[derive(Debug)]
pub struct InputSetzeAudio {
    pub satz_id: i32,
    pub file_path: PathBuf,
    pub voice_id: EnumVoiceIDElevenLabs,
}

impl From<InputSetzeAudio> for SqlSetzeAudio {
    fn from(value: InputSetzeAudio) -> Self {
        Self {
            satz_id: value.satz_id,
            file_path: value.file_path.to_string_lossy().into_owned(),
            voice_id: value.voice_id.get_key(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_setze_audio_into_sql_setze_audio_maps_fields() {
        let voice = EnumVoiceIDElevenLabs::GermanMan;

        let input = InputSetzeAudio {
            satz_id: 42,
            file_path: PathBuf::from("audios/setze/satz_000042_de.mp3"),
            voice_id: voice,
        };

        let sql: SqlSetzeAudio = input.into();

        assert_eq!(sql.satz_id, 42);
        assert_eq!(sql.file_path, "audios/setze/satz_000042_de.mp3");
        assert_eq!(sql.voice_id, voice.get_key());
    }

    #[test]
    fn input_setze_audio_path_is_string_lossy() {
        let voice = EnumVoiceIDElevenLabs::SpanishWoman;

        let input = InputSetzeAudio {
            satz_id: 1,
            file_path: PathBuf::from("/tmp/áéíóú.mp3"),
            voice_id: voice,
        };

        let sql: SqlSetzeAudio = input.into();

        assert_eq!(sql.satz_id, 1);
        assert_eq!(sql.file_path, "/tmp/áéíóú.mp3");
        assert_eq!(sql.voice_id, voice.get_key());
    }
}
