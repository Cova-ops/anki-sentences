use std::{collections::HashMap, str::FromStr};

use reqwest::{blocking::Client, header::HeaderMap};
use serde::Serialize;

use crate::helpers::error_handler::{ApiError, AppError, AppErrorKind, InvalidValueError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            "TX3LPaxmHKxFdv7VOQHJ" => Ok(Self::GermanMan),
            "EXAVITQu4vr4xnSDxMaL" => Ok(Self::SpanishWoman),

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

pub fn generate_tts(text: &str, voice_choice: LanguageVoice) -> Result<Vec<u8>, AppError> {
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

    let api_key = std::env::var("ELEVENLABS_API_KEY").map_err(|e| AppError {
        kind: AppErrorKind::Internal(format!("Unable to find ELEVENLABS_API_KEY on .env: {}", e)),
        context: vec![],
    })?;

    let headers = HashMap::from([
        (String::from("xi-api-key"), api_key),
        (
            String::from("Content-Type"),
            String::from("application/json"),
        ),
    ]);

    let res = client
        .post(&url)
        .headers(HeaderMap::try_from(&headers).map_err(|e| AppError {
            kind: AppErrorKind::Internal(format!("Error convertir HashMap to HeaderMap: {}", e)),
            context: vec![],
        })?)
        .json(&body)
        .send()
        .map_err(|e| ApiError {
            url: Some(url.clone()),
            headers: headers.clone(),
            method: String::from("POST"),
            payload: Some(
                serde_json::to_string_pretty(&body)
                    .map_err(|e| AppError {
                        kind: AppErrorKind::Internal(format!("Error serialize json: {e}")),
                        context: vec![],
                    })
                    .unwrap(),
            ),
            response: None,
            status: e.status().map(|d| d.to_string()),
        })?;

    let bytes = res.bytes().map_err(|e| ApiError {
        url: Some(url),
        headers,
        method: String::from("POST"),
        payload: Some(
            serde_json::to_string_pretty(&body)
                .map_err(|e| AppError {
                    kind: AppErrorKind::Internal(format!("Error serialize json: {e}")),
                    context: vec![],
                })
                .unwrap(),
        ),
        response: None,
        status: e.status().map(|d| d.to_string()),
    })?;

    Ok(bytes.to_vec())
}
