use crate::{
    console::cli::ReviewCmd,
    helpers::{error_handler::AppError, toml::AppConfig},
    services::tts::eleven_labs::LanguageVoice,
};

mod setze;
mod wort;

pub fn run(cmd: ReviewCmd, config: &AppConfig) -> Result<(), AppError> {
    match cmd {
        ReviewCmd::WorteEs {
            section,
            batch,
            no_shuffle,
        } => wort::run(config, section, batch, no_shuffle, LanguageVoice::Spanisch)?,
        ReviewCmd::WorteDe {
            section,
            batch,
            no_shuffle,
        } => wort::run(config, section, batch, no_shuffle, LanguageVoice::Deutsch)?,
        ReviewCmd::Setze {
            section,
            batch,
            no_shuffle,
        } => setze::run(config, section, batch, no_shuffle)?,
    };

    Ok(())
}
