use std::{path::PathBuf, str::FromStr};

use chrono::{DateTime, Utc};

use crate::{
    db::schemas::setze_audio::SchemaSetzeAudio,
    helpers::{error_handler::InvalidValueError, time::string_2_datetime},
    services::tts::eleven_labs::EnumVoiceIDElevenLabs,
};

#[derive(Debug, Clone)]
pub struct ModelSetzeAudio {
    pub satz_id: i32,
    pub file_path: PathBuf,
    pub voice_id: EnumVoiceIDElevenLabs,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TryFrom<SchemaSetzeAudio> for ModelSetzeAudio {
    type Error = Vec<InvalidValueError>;

    fn try_from(value: SchemaSetzeAudio) -> Result<Self, Self::Error> {
        let mut errs = vec![];

        let satz_id = value.satz_id;
        let file_path = match PathBuf::from_str(&value.file_path) {
            Ok(v) => Some(v),
            Err(_) => {
                errs.push(InvalidValueError {
                    field: "file_path",
                    message: format!("{} can't be convert to file_path", value.file_path),
                    valid_options: None,
                });
                None
            }
        };

        let voice_id = match EnumVoiceIDElevenLabs::from_str(&value.voice_id) {
            Ok(v) => Some(v),
            Err(e) => {
                errs.push(e);
                None
            }
        };

        let created_at = match string_2_datetime(value.created_at) {
            Ok(v) => Some(v),
            Err(e) => {
                errs.push(e);
                None
            }
        };

        let deleted_at = if let Some(date) = value.deleted_at {
            match string_2_datetime(date) {
                Ok(v) => Some(v.clone()),
                Err(e) => {
                    errs.push(e);
                    None
                }
            }
        } else {
            None
        };

        if !errs.is_empty() {
            return Err(errs);
        }

        Ok(Self {
            satz_id,
            file_path: file_path.unwrap(),
            voice_id: voice_id.unwrap(),
            created_at: created_at.unwrap(),
            deleted_at,
        })
    }
}

impl ModelSetzeAudio {
    pub fn try_from_iter(
        value: impl IntoIterator<Item = SchemaSetzeAudio>,
    ) -> Result<Vec<ModelSetzeAudio>, Vec<InvalidValueError>> {
        let mut errs = vec![];
        let mut oks = vec![];

        for v in value {
            match ModelSetzeAudio::try_from(v) {
                Ok(v) => oks.push(v),
                Err(mut e) => errs.append(&mut e),
            }
        }

        if errs.is_empty() { Ok(oks) } else { Err(errs) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schema_ok_german(deleted_at: Option<&str>) -> SchemaSetzeAudio {
        SchemaSetzeAudio {
            satz_id: 10,
            file_path: "audios/setze/10.mp3".to_string(),
            voice_id: EnumVoiceIDElevenLabs::GermanMan.get_key().to_string(),
            created_at: "2025-01-01 12:00:00".to_string(),
            deleted_at: deleted_at.map(|s| s.to_string()),
        }
    }

    #[test]
    fn voice_id_from_str_ok() {
        assert_eq!(
            EnumVoiceIDElevenLabs::from_str("TX3LPaxmHKxFdv7VOQHJ").unwrap(),
            EnumVoiceIDElevenLabs::GermanMan
        );
        assert_eq!(
            EnumVoiceIDElevenLabs::from_str("EXAVITQu4vr4xnSDxMaL").unwrap(),
            EnumVoiceIDElevenLabs::SpanishWoman
        );
    }

    #[test]
    fn voice_id_from_str_err() {
        let err = EnumVoiceIDElevenLabs::from_str("INVALID_KEY").unwrap_err();
        assert_eq!(err.field, "VoiceId");
        assert!(err.message.contains("INVALID_KEY"));
        assert!(err.valid_options.is_none());
    }

    #[test]
    fn model_try_from_ok_without_deleted_at() {
        let schema = schema_ok_german(None);

        let model = ModelSetzeAudio::try_from(schema).unwrap();

        assert_eq!(model.satz_id, 10);
        assert_eq!(model.file_path, PathBuf::from("audios/setze/10.mp3"));
        assert_eq!(model.voice_id, EnumVoiceIDElevenLabs::GermanMan);

        // No comparo DateTime exacto si no quieres, pero aquí sí se puede:
        assert_eq!(
            model.created_at,
            string_2_datetime("2025-01-01 12:00:00").unwrap()
        );
        assert_eq!(model.deleted_at, None);
    }

    #[test]
    fn model_try_from_ok_with_deleted_at() {
        let schema = schema_ok_german(Some("2025-01-10 00:00:00"));

        let model = ModelSetzeAudio::try_from(schema).unwrap();

        assert_eq!(
            model.deleted_at,
            Some(string_2_datetime("2025-01-10 00:00:00").unwrap())
        );
    }

    #[test]
    fn model_try_from_collects_errors_voice_id_and_datetime() {
        let schema = SchemaSetzeAudio {
            satz_id: 11,
            file_path: "audios/setze/11.mp3".to_string(),
            voice_id: "BAD_KEY".to_string(),
            created_at: "NOT_A_DATE".to_string(),
            deleted_at: None,
        };

        let errs = ModelSetzeAudio::try_from(schema).unwrap_err();

        // Debe traer ambos errores (VoiceId + datetime)
        assert!(errs.iter().any(|e| e.field == "VoiceId"));
        assert!(errs.iter().any(|e| e.field == "datetime"));
    }

    #[test]
    fn model_try_from_iter_ok_all() {
        let data = vec![
            schema_ok_german(None),
            SchemaSetzeAudio {
                satz_id: 12,
                file_path: "audios/setze/12.mp3".to_string(),
                voice_id: EnumVoiceIDElevenLabs::SpanishWoman.get_key().to_string(),
                created_at: "2025-01-02 10:00:00".to_string(),
                deleted_at: None,
            },
        ];

        let res = ModelSetzeAudio::try_from_iter(data).unwrap();
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].voice_id, EnumVoiceIDElevenLabs::GermanMan);
        assert_eq!(res[1].voice_id, EnumVoiceIDElevenLabs::SpanishWoman);
    }

    #[test]
    fn model_try_from_iter_err_aggregates_all_errors() {
        let data = vec![
            schema_ok_german(None),
            SchemaSetzeAudio {
                satz_id: 99,
                file_path: "audios/setze/99.mp3".to_string(),
                voice_id: "BAD_KEY".to_string(),
                created_at: "NOT_A_DATE".to_string(),
                deleted_at: Some("ALSO_BAD_DATE".to_string()),
            },
        ];

        let errs = ModelSetzeAudio::try_from_iter(data).unwrap_err();

        // Debe contener VoiceId y al menos un datetime (created_at) + otro datetime (deleted_at)
        assert!(errs.iter().any(|e| e.field == "VoiceId"));
        assert!(errs.iter().any(|e| e.field == "datetime"));
        // usualmente serán 3 errores aquí (voice + created_at + deleted_at)
        assert!(errs.len() >= 2);
    }

    #[test]
    fn file_path_is_parsed_to_pathbuf() {
        // Este test existe porque tu rama Err(_) en PathBuf::from_str casi nunca se dispara.
        let schema = SchemaSetzeAudio {
            satz_id: 13,
            file_path: "weird path with spaces/and_üñíçødé.mp3".to_string(),
            voice_id: EnumVoiceIDElevenLabs::GermanMan.get_key().to_string(),
            created_at: "2025-01-03 00:00:00".to_string(),
            deleted_at: None,
        };

        let model = ModelSetzeAudio::try_from(schema).unwrap();
        assert_eq!(
            model.file_path,
            PathBuf::from("weird path with spaces/and_üñíçødé.mp3")
        );
    }
}
