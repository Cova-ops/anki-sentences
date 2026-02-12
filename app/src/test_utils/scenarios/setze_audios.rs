use crate::{
    db::schemas::setze_audio::InputSetzeAudio, services::tts::eleven_labs::EnumVoiceIDElevenLabs,
    test_utils::scenarios::Scenario,
};

pub fn scenario_setze_audio() -> Scenario<InputSetzeAudio> {
    Scenario {
        initial: vec![
            InputSetzeAudio {
                satz_id: 1,
                file_path: PathBuf::from("temp"),
                voice: EnumVoiceIDElevenLabs::GermanMan,
            },
            InputSetzeAudio {
                satz_id: 2,
                file_path: PathBuf::from("user"),
                voice: EnumVoiceIDElevenLabs::SpanishWoman,
            },
        ],
        update: vec![
            InputSetzeAudio {
                satz_id: 1,
                file_path: PathBuf::from("temp_test"),
                voice: EnumVoiceIDElevenLabs::SpanishWoman,
            },
            InputSetzeAudio {
                satz_id: 2,
                file_path: PathBuf::from("user_test"),
                voice: EnumVoiceIDElevenLabs::GermanMan,
            },
        ],
    }
}
