use crate::{db::schemas::setze_audio::InputSetzeAudio, test_utils::scenarios::Scenario};

pub fn scenario_setze_audio() -> Scenario<InputSetzeAudio> {
    Scenario {
        initial: vec![
            InputSetzeAudio {
                satz_id: 1,
                audio_name_es: None,
                audio_name_de: Some(String::from("audio_de_1")),
            },
            InputSetzeAudio {
                satz_id: 2,
                audio_name_es: Some(String::from("audio_es_2")),
                audio_name_de: None,
            },
            InputSetzeAudio {
                satz_id: 3,
                audio_name_es: Some(String::from("audio_es_3")),
                audio_name_de: Some(String::from("audio_de_3")),
            },
            InputSetzeAudio {
                satz_id: 4,
                audio_name_es: None,
                audio_name_de: None,
            },
        ],
        update: vec![
            InputSetzeAudio {
                satz_id: 1,
                audio_name_es: Some(String::from("audio_es_1_test")),
                audio_name_de: Some(String::from("audio_de_1_test")),
            },
            InputSetzeAudio {
                satz_id: 2,
                audio_name_es: Some(String::from("audio_es_2_test")),
                audio_name_de: Some(String::from("audio_de_2_test")),
            },
            InputSetzeAudio {
                satz_id: 3,
                audio_name_es: Some(String::from("audio_es_3_test")),
                audio_name_de: Some(String::from("audio_de_3_test")),
            },
            InputSetzeAudio {
                satz_id: 4,
                audio_name_es: None,
                audio_name_de: None,
            },
        ],
        update_id: vec![],
    }
}
