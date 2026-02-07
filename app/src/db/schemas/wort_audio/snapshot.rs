use crate::db::schemas::wort_audio::ModelWortAudio;

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

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
