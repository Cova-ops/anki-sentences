use crate::{
    db::view::wort_audio_missing::SchemaWortAudioMissing, helpers::error_handler::InvalidValueError,
};

#[derive(Debug)]
pub struct ModelWortAudioMissing {
    pub id: i32,
    pub wort_es: String,
    pub wort_de: String,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,
}

impl TryFrom<SchemaWortAudioMissing> for ModelWortAudioMissing {
    type Error = Vec<InvalidValueError>;

    fn try_from(value: SchemaWortAudioMissing) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            wort_es: value.wort_es,
            wort_de: value.wort_de,
            audio_name_es: value.audio_name_es,
            audio_name_de: value.audio_name_de,
        })
    }
}

#[cfg(test)]
mod tests_model_wort_audio_missing {
    use super::*;

    #[test]
    fn try_from_schema_maps_all_fields_some() {
        let schema = SchemaWortAudioMissing {
            id: 10,
            wort_es: "perro".to_string(),
            wort_de: "Hund".to_string(),
            audio_name_es: Some("perro.mp3".to_string()),
            audio_name_de: Some("hund.mp3".to_string()),
        };

        let model = ModelWortAudioMissing::try_from(schema).unwrap();

        assert_eq!(model.id, 10);
        assert_eq!(model.wort_es, "perro");
        assert_eq!(model.wort_de, "Hund");
        assert_eq!(model.audio_name_es.as_deref(), Some("perro.mp3"));
        assert_eq!(model.audio_name_de.as_deref(), Some("hund.mp3"));
    }

    #[test]
    fn try_from_schema_maps_all_fields_none() {
        let schema = SchemaWortAudioMissing {
            id: 11,
            wort_es: "gato".to_string(),
            wort_de: "Katze".to_string(),
            audio_name_es: None,
            audio_name_de: None,
        };

        let model = ModelWortAudioMissing::try_from(schema).unwrap();

        assert_eq!(model.id, 11);
        assert_eq!(model.wort_es, "gato");
        assert_eq!(model.wort_de, "Katze");
        assert!(model.audio_name_es.is_none());
        assert!(model.audio_name_de.is_none());
    }
}
