#[derive(Debug)]
pub struct SqlWortAudio {
    pub wort_id: i32,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,
}

#[derive(Debug)]
pub struct InputWortAudio {
    pub wort_id: i32,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,
}

impl From<InputWortAudio> for SqlWortAudio {
    fn from(value: InputWortAudio) -> Self {
        Self {
            wort_id: value.wort_id,
            audio_name_es: value.audio_name_es,
            audio_name_de: value.audio_name_de,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_input_to_sql_all_some() {
        let input = InputWortAudio {
            wort_id: 42,
            audio_name_es: Some("hola_es.mp3".to_string()),
            audio_name_de: Some("hallo_de.mp3".to_string()),
        };

        let sql = SqlWortAudio::from(input);

        assert_eq!(sql.wort_id, 42);
        assert_eq!(sql.audio_name_es, Some("hola_es.mp3".to_string()));
        assert_eq!(sql.audio_name_de, Some("hallo_de.mp3".to_string()));
    }

    #[test]
    fn from_input_to_sql_one_none() {
        let input = InputWortAudio {
            wort_id: 7,
            audio_name_es: Some("solo_es.mp3".to_string()),
            audio_name_de: None,
        };

        let sql = SqlWortAudio::from(input);

        assert_eq!(sql.wort_id, 7);
        assert_eq!(sql.audio_name_es, Some("solo_es.mp3".to_string()));
        assert_eq!(sql.audio_name_de, None);
    }

    #[test]
    fn from_input_to_sql_all_none() {
        let input = InputWortAudio {
            wort_id: 99,
            audio_name_es: None,
            audio_name_de: None,
        };

        let sql = SqlWortAudio::from(input);

        assert_eq!(sql.wort_id, 99);
        assert_eq!(sql.audio_name_es, None);
        assert_eq!(sql.audio_name_de, None);
    }
}
