use crate::db::schemas::setze_audio::{ModelSetzeAudio, SchemaSetzeAudio};

#[derive(Debug)]
pub struct SnapshotSetzeAudio {
    pub satz_id: i32,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,

    // Generic
    pub created_at: &'static str,
    pub deleted_at: Option<&'static str>,
}

impl From<ModelSetzeAudio> for SnapshotSetzeAudio {
    fn from(value: ModelSetzeAudio) -> Self {
        Self {
            satz_id: value.satz_id,
            audio_name_es: value.audio_name_es,
            audio_name_de: value.audio_name_de,

            created_at: "<created_at>",
            deleted_at: value.deleted_at.as_ref().map(|_| "<deleted_at>"),
        }
    }
}

impl From<SchemaSetzeAudio> for SnapshotSetzeAudio {
    /// Don't use this on prod
    /// it doesn't handle errors
    fn from(value: SchemaSetzeAudio) -> Self {
        let model = ModelSetzeAudio::try_from(value).unwrap();
        model.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    mod from_model {
        use super::*;

        #[test]
        fn without_deleted_at() {
            let model = ModelSetzeAudio {
                satz_id: 10,
                audio_name_es: Some("audio_es.mp3".to_string()),
                audio_name_de: Some("audio_de.mp3".to_string()),
                created_at: Utc.with_ymd_and_hms(2025, 12, 4, 17, 44, 37).unwrap(),
                deleted_at: None,
            };

            let snap = SnapshotSetzeAudio::from(model);

            assert_eq!(snap.satz_id, 10);
            assert_eq!(snap.audio_name_es.as_deref(), Some("audio_es.mp3"));
            assert_eq!(snap.audio_name_de.as_deref(), Some("audio_de.mp3"));
            assert_eq!(snap.created_at, "<created_at>");
            assert_eq!(snap.deleted_at, None);
        }

        #[test]
        fn with_deleted_at() {
            let model = ModelSetzeAudio {
                satz_id: 11,
                audio_name_es: Some("audio_es.mp3".to_string()),
                audio_name_de: Some("audio_de.mp3".to_string()),
                created_at: Utc.with_ymd_and_hms(2025, 12, 4, 17, 44, 37).unwrap(),
                deleted_at: Some(Utc.with_ymd_and_hms(2025, 12, 5, 10, 0, 0).unwrap()),
            };

            let snap = SnapshotSetzeAudio::from(model);

            assert_eq!(snap.satz_id, 11);
            assert_eq!(snap.audio_name_es.as_deref(), Some("audio_es.mp3"));
            assert_eq!(snap.audio_name_de.as_deref(), Some("audio_de.mp3"));
            assert_eq!(snap.created_at, "<created_at>");
            assert_eq!(snap.deleted_at, Some("<deleted_at>"));
        }
    }

    mod from_schema {
        use super::*;

        #[test]
        fn happy_path() {
            let schema = SchemaSetzeAudio {
                satz_id: 10,
                audio_name_es: Some("audio_es.mp3".to_string()),
                audio_name_de: Some("audio_de.mp3".to_string()),
                created_at: "2026-02-10 10:00:00".to_string(),
                deleted_at: None,
            };

            let snap: SnapshotSetzeAudio = schema.into();

            assert_eq!(snap.satz_id, 10);
            assert_eq!(snap.audio_name_es, Some(String::from("audio_es.mp3")));
            assert_eq!(snap.audio_name_de, Some(String::from("audio_de.mp3")));
            assert_eq!(snap.created_at, "<created_at>");
            assert_eq!(snap.deleted_at, None);
        }

        #[test]
        fn panics_on_invalid_schema() {
            let invalid = SchemaSetzeAudio {
                satz_id: 1,
                audio_name_es: Some("audio_es.mp3".to_string()),
                audio_name_de: Some("audio_de.mp3".to_string()),
                created_at: "NOT_A_DATE".to_string(),
                deleted_at: None,
            };

            let result = std::panic::catch_unwind(|| {
                let _: SnapshotSetzeAudio = invalid.into();
            });

            assert!(
                result.is_err(),
                "expected conversion to panic due to unwrap()"
            );
        }
    }
}
