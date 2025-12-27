use color_eyre::eyre::Result;
use reqwest::blocking::Client;
use serde::Serialize;

static VOICE_ID_DE_MASC: &str = "g1jpii0iyvtRs8fqXsd1";
static VOICE_ID_ES_FEME: &str = "zl1Ut8dvwcVSuQSB9XkG";

#[derive(Serialize)]
struct ElevenRequest<'a> {
    text: &'a str,
    model_id: &'a str,
    voice_settings: VoiceSettings,
}

#[derive(Serialize)]
struct VoiceSettings {
    stability: f32,
    similarity_boost: f32,
    style: f32,
    use_speaker_boost: bool,
}

#[derive(Debug)]
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
        LanguageVoice::Deutsch => VOICE_ID_DE_MASC,
        LanguageVoice::Spanisch => VOICE_ID_ES_FEME,
    };

    let url = format!(
        "https://api.elevenlabs.io/v1/text-to-speech/{}?output_format=mp3_22050_32",
        voice
    );

    let client = Client::new();

    let body = ElevenRequest {
        text,
        model_id: "eleven_flash_v2_5",
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
        .error_for_status()?; // Si falla, truena con error bonito

    let bytes = res.bytes()?.to_vec();
    Ok(bytes)
}
