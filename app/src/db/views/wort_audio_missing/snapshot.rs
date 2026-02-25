use crate::db::views::wort_audio_missing::{ModelWortAudioMissing, SchemaWortAudioMissing};

#[derive(Debug)]
pub struct SnapshotWortAudioMissing {
    pub id: i32,
    pub wort_es: String,
    pub wort_de: String,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,
}

impl From<ModelWortAudioMissing> for SnapshotWortAudioMissing {
    fn from(value: ModelWortAudioMissing) -> Self {
        Self {
            id: value.id,
            wort_es: value.wort_es,
            wort_de: value.wort_de,
            audio_name_es: value.audio_name_es,
            audio_name_de: value.audio_name_de,
        }
    }
}

impl From<SchemaWortAudioMissing> for SnapshotWortAudioMissing {
    /// Don't use this in prod
    /// It doesn't handle errors
    fn from(value: SchemaWortAudioMissing) -> Self {
        let model = ModelWortAudioMissing::try_from(value).unwrap();
        model.into()
    }
}

#[cfg(test)]
mod tests_snapshot_wort_audio_missing {
    use super::*;

    mod from_model {
        use super::*;

        #[test]
        fn from_model_maps_all_fields_some() {
            let model = ModelWortAudioMissing {
                id: 42,
                wort_es: "casa".to_string(),
                wort_de: "Haus".to_string(),
                audio_name_es: Some("casa.mp3".to_string()),
                audio_name_de: Some("haus.mp3".to_string()),
            };

            let snapshot = SnapshotWortAudioMissing::from(model);

            assert_eq!(snapshot.id, 42);
            assert_eq!(snapshot.wort_es, "casa");
            assert_eq!(snapshot.wort_de, "Haus");
            assert_eq!(snapshot.audio_name_es.as_deref(), Some("casa.mp3"));
            assert_eq!(snapshot.audio_name_de.as_deref(), Some("haus.mp3"));
        }

        #[test]
        fn from_model_maps_all_fields_none() {
            let model = ModelWortAudioMissing {
                id: 43,
                wort_es: "árbol".to_string(),
                wort_de: "Baum".to_string(),
                audio_name_es: None,
                audio_name_de: None,
            };

            let snapshot = SnapshotWortAudioMissing::from(model);

            assert_eq!(snapshot.id, 43);
            assert_eq!(snapshot.wort_es, "árbol");
            assert_eq!(snapshot.wort_de, "Baum");
            assert!(snapshot.audio_name_es.is_none());
            assert!(snapshot.audio_name_de.is_none());
        }
    }

    mod from_schema {
        use super::*;

        #[test]
        fn happy_path() {
            let schema = SnapshotWortAudioMissing {
                id: 42,
                wort_es: "casa".to_string(),
                wort_de: "Haus".to_string(),
                audio_name_es: Some("casa.mp3".to_string()),
                audio_name_de: Some("haus.mp3".to_string()),
            };

            let snap: SnapshotWortAudioMissing = schema.into();

            assert_eq!(snap.id, 42);
            assert_eq!(snap.wort_es, String::from("casa"));
            assert_eq!(snap.wort_de, String::from("Haus"));
            assert_eq!(snap.audio_name_es, Some(String::from("casa.mp3")));
            assert_eq!(snap.audio_name_de, Some(String::from("haus.mp3")));
        }

        #[test]
        fn panics_on_invalid_schema() {
            // SAFETY: ModelWortAudioMissing::try_from is infallible (always returns Ok).
            // The Result signature is kept only for consistency with other models.}
        }
    }
}
