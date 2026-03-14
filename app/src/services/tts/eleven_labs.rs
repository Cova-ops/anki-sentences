use reqwest::{blocking::Client, header::HeaderMap};
use serde::Serialize;

use crate::{
    helpers::error_handler::{ApiError, AppError},
    services::tts::language_voice::LanguageVoice,
};

impl LanguageVoice {
    pub fn get_eleven_key(&self) -> &'static str {
        match self {
            Self::Deutsch => "TX3LPaxmHKxFdv7VOQHJ",
            Self::Spanisch => "EXAVITQu4vr4xnSDxMaL",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_eleven_key {
        use super::*;

        #[test]
        fn ok_deutsch_key() {
            let lang = LanguageVoice::Deutsch;
            let key = lang.get_eleven_key();

            assert_eq!(key, "TX3LPaxmHKxFdv7VOQHJ");
        }

        #[test]
        fn ok_spanisch_key() {
            let lang = LanguageVoice::Spanisch;
            let key = lang.get_eleven_key();

            assert_eq!(key, "EXAVITQu4vr4xnSDxMaL");
        }
    }
}

#[derive(Serialize)]
struct BodyRequest<'a> {
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

pub fn generate_tts(text: &str, voice_choice: LanguageVoice) -> Result<Vec<u8>, AppError> {
    let voice = voice_choice.get_eleven_key();
    let url = format!(
        "https://api.elevenlabs.io/v1/text-to-speech/{}?output_format=mp3_22050_32",
        voice
    );

    let client = Client::new();

    let body = BodyRequest {
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
    let body_string: String = serde_json::to_string(&body)?;
    let api_key = std::env::var("ELEVENLABS_API_KEY")?;

    let mut headers = HeaderMap::new();
    headers.insert("xi-api-key", api_key.parse()?);
    headers.insert("Content-Type", "application/json".try_into()?);

    let res = client
        .post(&url)
        .headers(headers.clone())
        .json(&body)
        .send()
        .map_err(|e| ApiError {
            url: Some(url.clone()),
            headers: headers.clone(),
            method: String::from("POST"),
            payload: Some(body_string.clone()),
            response: None,
            status: e.status().map(|d| d.to_string()),
        })?;

    let bytes = res.bytes().map_err(|e| ApiError {
        url: Some(url),
        headers,
        method: String::from("POST"),
        payload: Some(body_string),
        response: None,
        status: e.status().map(|d| d.to_string()),
    })?;

    Ok(bytes.to_vec())
}
