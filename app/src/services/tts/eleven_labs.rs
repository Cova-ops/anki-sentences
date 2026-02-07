use std::str::FromStr;

use color_eyre::eyre::Result;
use reqwest::blocking::Client;
use serde::Serialize;

use crate::helpers::error_handler::InvalidValueError;

pub enum EnumVoiceIDElevenLabs {
    GermanMan,
    SpanishWoman,
}

impl EnumVoiceIDElevenLabs {
    pub fn get_key(&self) -> &'static str {
        match self {
            Self::GermanMan => "TX3LPaxmHKxFdv7VOQHJ",
            Self::SpanishWoman => "EXAVITQu4vr4xnSDxMaL",
        }
    }
}

impl FromStr for EnumVoiceIDElevenLabs {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "TX3LPaxmHKxFdv7VOQHJ" => Self::GermanMan,
            "EXAVITQu4vr4xnSDxMaL" => Self::SpanishWoman,

            _ => {
                return Err(InvalidValueError {
                    field: "VoiceId",
                    message: format!("{s} cannot be convert to VoiceId"),
                    valid_options: None, // We don't show the valid keys for security
                });
            }
        }
    }
}

#[derive(Serialize)]
struct ElevenRequest<'a> {
    text: &'a str,
    model_id: &'a str,
    language_code: &'a str,
    voice_settings: VoiceSettings,
}

#[derive(Serialize)]
struct VoiceSettings {
    stability: f32,
    similarity_boost: f32,
    style: f32,
    use_speaker_boost: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LanguageVoice {
    Deutsch,
    Spanisch,
}

impl LanguageVoice {
    pub fn get_posfix(&self) -> String {
        match self {
            LanguageVoice::Spanisch => "es".to_owned(),
            LanguageVoice::Deutsch => "de".to_owned(),
        }
    }
}

pub fn generate_tts(text: &str, voice_choice: LanguageVoice) -> Result<Vec<u8>> {
    let voice = match voice_choice {
        LanguageVoice::Deutsch => EnumVoiceIDElevenLabs::GermanMan.get_key(),
        LanguageVoice::Spanisch => EnumVoiceIDElevenLabs::SpanishWoman.get_key(),
    };

    let url = format!(
        "https://api.elevenlabs.io/v1/text-to-speech/{}?output_format=mp3_22050_32",
        voice
    );

    let client = Client::new();

    let body = ElevenRequest {
        text: &format!("{}.", text),
        model_id: "eleven_flash_v2_5",
        language_code: &voice_choice.get_posfix(),
        voice_settings: VoiceSettings {
            stability: 0.6,
            similarity_boost: 0.8,
            style: 0.4,
            use_speaker_boost: true,
        },
    };

    let res = client
        .post(&url)
        .header("xi-api-key", std::env::var("ELEVENLABS_API_KEY")?)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()?
        .error_for_status()?; // If fails, this make a pretty display error

    let bytes = res.bytes()?.to_vec();
    Ok(bytes)
}
