use crate::db::schemas::wort_audio::InputWortAudio;

pub fn scenario_wort_audio() -> Scenario<InputWortAudio> {
    Scenario {
        initial: vec![
            InputWortAudio {
                wort_id: 1,
                audio_name_es: None,
                audio_name_de: Some("audio_de_1"),
            },
            InputWortAudio {
                wort_id: 2,
                audio_name_es: Some("audio_es_2"),
                audio_name_de: None,
            },
            InputWortAudio {
                wort_id: 3,
                audio_name_es: Some("audio_es_3"),
                audio_name_de: Some("audio_de_3"),
            },
            InputWortAudio {
                wort_id: 4,
                audio_name_es: None,
                audio_name_de: None,
            },
        ],
        update: vec![
            InputWortAudio {
                wort_id: 1,
                audio_name_es: Some("audio_es_1_test"),
                audio_name_de: Some("audio_de_1_test"),
            },
            InputWortAudio {
                wort_id: 2,
                audio_name_es: Some("audio_es_2_test"),
                audio_name_de: Some("audio_de_2_test"),
            },
            InputWortAudio {
                wort_id: 3,
                audio_name_es: Some("audio_es_3_test"),
                audio_name_de: Some("audio_de_3_test"),
            },
            InputWortAudio {
                wort_id: 4,
                audio_name_es: None,
                audio_name_de: None,
            },
        ],
    }
}
