use std::path::PathBuf;

use crate::{
    db::schemas::setze_audio::ModelSetzeAudio, services::tts::eleven_labs::EnumVoiceIDElevenLabs,
};

#[derive(Debug)]
pub struct SnapshotSetzeAudio {
    pub satz_id: i32,
    pub file_path: PathBuf,
    pub voice_id: EnumVoiceIDElevenLabs,

    // Generic
    pub created_at: &'static str,
    pub deleted_at: Option<&'static str>,
}

impl From<ModelSetzeAudio> for SnapshotSetzeAudio {
    fn from(value: ModelSetzeAudio) -> Self {
        Self {
            satz_id: value.satz_id,
            file_path: value.file_path,
            voice_id: value.voice_id,

            created_at: "<created_at>",
            deleted_at: value.deleted_at.as_ref().map(|_| "<deleted_at>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn snapshot_setze_audio_from_model_without_deleted_at() {
        let model = ModelSetzeAudio {
            satz_id: 10,
            file_path: PathBuf::from("audios/setze/satz_000010_es.mp3"),
            voice_id: EnumVoiceIDElevenLabs::SpanishWoman,
            created_at: Utc.with_ymd_and_hms(2025, 12, 4, 17, 44, 37).unwrap(),
            deleted_at: None,
        };

        let snap = SnapshotSetzeAudio::from(model);

        assert_eq!(snap.satz_id, 10);
        assert_eq!(
            snap.file_path,
            PathBuf::from("audios/setze/satz_000010_es.mp3")
        );
        assert_eq!(snap.voice_id, EnumVoiceIDElevenLabs::SpanishWoman);
        assert_eq!(snap.created_at, "<created_at>");
        assert_eq!(snap.deleted_at, None);
    }

    #[test]
    fn snapshot_setze_audio_from_model_with_deleted_at() {
        let model = ModelSetzeAudio {
            satz_id: 11,
            file_path: PathBuf::from("audios/setze/satz_000011_de.mp3"),
            voice_id: EnumVoiceIDElevenLabs::GermanMan,
            created_at: Utc.with_ymd_and_hms(2025, 12, 4, 17, 44, 37).unwrap(),
            deleted_at: Some(Utc.with_ymd_and_hms(2025, 12, 5, 10, 0, 0).unwrap()),
        };

        let snap = SnapshotSetzeAudio::from(model);

        assert_eq!(snap.satz_id, 11);
        assert_eq!(
            snap.file_path,
            PathBuf::from("audios/setze/satz_000011_de.mp3")
        );
        assert_eq!(snap.voice_id, EnumVoiceIDElevenLabs::GermanMan);
        assert_eq!(snap.created_at, "<created_at>");
        assert_eq!(snap.deleted_at, Some("<deleted_at>"));
    }
}
