use crate::db::schemas::wort_audio::{ModelWortAudio, SchemaWortAudio};

#[derive(Debug, Clone)]
pub struct SnapshotWortAudio {
    pub wort_id: i32,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,

    // Generic
    pub created_at: &'static str,
    pub deleted_at: Option<&'static str>,
}

impl From<ModelWortAudio> for SnapshotWortAudio {
    fn from(value: ModelWortAudio) -> Self {
        Self {
            wort_id: value.wort_id,
            audio_name_es: value.audio_name_es,
            audio_name_de: value.audio_name_de,

            created_at: "<created_at>",
            deleted_at: value.deleted_at.as_ref().map(|_| "<deleted_at>"),
        }
    }
}

impl From<SchemaWortAudio> for SnapshotWortAudio {
    /// Don't use this in prod
    /// It doesn't handle errors
    fn from(value: SchemaWortAudio) -> Self {
        let model = ModelWortAudio::try_from(value).unwrap();
        model.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

    mod from_model {
        use super::*;

        fn model_ok(deleted: bool) -> ModelWortAudio {
            ModelWortAudio {
                wort_id: 42,
                audio_name_es: Some("audio_es.mp3".to_string()),
                audio_name_de: Some("audio_de.mp3".to_string()),
                created_at: DateTime::<Utc>::from_timestamp(1_700_000_000, 0).unwrap(),
                deleted_at: if deleted {
                    Some(DateTime::<Utc>::from_timestamp(1_700_000_100, 0).unwrap())
                } else {
                    None
                },
            }
        }

        #[test]
        fn snapshot_from_model_without_deleted_at() {
            let model = model_ok(false);

            let snap = SnapshotWortAudio::from(model);

            assert_eq!(snap.wort_id, 42);
            assert_eq!(snap.audio_name_es.as_deref(), Some("audio_es.mp3"));
            assert_eq!(snap.audio_name_de.as_deref(), Some("audio_de.mp3"));

            assert_eq!(snap.created_at, "<created_at>");
            assert!(snap.deleted_at.is_none());
        }

        #[test]
        fn snapshot_from_model_with_deleted_at() {
            let model = model_ok(true);

            let snap = SnapshotWortAudio::from(model);

            assert_eq!(snap.wort_id, 42);
            assert_eq!(snap.audio_name_es.as_deref(), Some("audio_es.mp3"));
            assert_eq!(snap.audio_name_de.as_deref(), Some("audio_de.mp3"));

            assert_eq!(snap.created_at, "<created_at>");
            assert_eq!(snap.deleted_at, Some("<deleted_at>"));
        }

        #[test]
        fn snapshot_preserves_none_audio_fields() {
            let model = ModelWortAudio {
                wort_id: 7,
                audio_name_es: None,
                audio_name_de: None,
                created_at: DateTime::<Utc>::from_timestamp(1_700_000_000, 0).unwrap(),
                deleted_at: None,
            };

            let snap = SnapshotWortAudio::from(model);

            assert_eq!(snap.wort_id, 7);
            assert!(snap.audio_name_es.is_none());
            assert!(snap.audio_name_de.is_none());
            assert_eq!(snap.created_at, "<created_at>");
            assert!(snap.deleted_at.is_none());
        }
    }

    mod from_schema {
        use super::*;

        #[test]
        fn happy_path() {
            let schema = SchemaWortAudio {
                wort_id: 42,
                audio_name_es: Some("audio_es.mp3".to_string()),
                audio_name_de: Some("audio_de.mp3".to_string()),
                created_at: String::from("2019-06-30 20:00:00"),
                deleted_at: None,
            };

            let snap: SnapshotWortAudio = schema.into();

            assert_eq!(snap.wort_id, 42);
            assert_eq!(snap.audio_name_es, Some(String::from("audio_es.mp3")));
            assert_eq!(snap.audio_name_de, Some(String::from("audio_de.mp3")));
            assert_eq!(snap.created_at, "<created_at>");
            assert_eq!(snap.deleted_at, None);
        }

        #[test]
        fn panics_on_invalid_schema() {
            let invalid = SchemaWortAudio {
                wort_id: 42,
                audio_name_es: Some("audio_es.mp3".to_string()),
                audio_name_de: Some("audio_de.mp3".to_string()),
                created_at: String::from("INVALID_DATE"),
                deleted_at: None,
            };

            let result = std::panic::catch_unwind(|| {
                let _: SnapshotWortAudio = invalid.into();
            });

            assert!(
                result.is_err(),
                "expected conversion to panic due to unwrap()"
            );
        }
    }
}
