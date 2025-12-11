use color_eyre::eyre::Result;
use reqwest::blocking::Client;
use serde::Serialize;

static API_KEY: &str = "sk_b67a171c654c0a189cf1d5ff5efc5cd953395363658ab244";
static VOICE_ID_DE_MASC: &str = "TX3LPaxmHKxFdv7VOQHJ";

#[derive(Serialize)]
struct ElevenRequest<'a> {
    text: &'a str,
    model_id: &'a str,
}

pub fn generate_tts(text: &str) -> Result<Vec<u8>> {
    let url = format!(
        "https://api.elevenlabs.io/v1/text-to-speech/{}?output_format=mp3_22050_32",
        VOICE_ID_DE_MASC
    );

    let client = Client::new();

    let body = ElevenRequest {
        text,
        model_id: "eleven_multilingual_v2",
    };

    let res = client
        .post(&url)
        .header("xi-api-key", API_KEY)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()?
        .error_for_status()?; // Si falla, truena con error bonito

    let bytes = res.bytes()?.to_vec();
    Ok(bytes)
}
