use color_eyre::Result;

use crate::{
    console::cli::ReviewCmd, helpers::toml::AppConfig, services::tts::eleven_labs::LanguageVoice,
};

mod setze;
mod worte;

pub fn run(cmd: ReviewCmd, config: &AppConfig) -> Result<()> {
    match cmd {
        ReviewCmd::WorteEs {
            section,
            batch,
            no_shuffle,
        } => worte::run(config, section, batch, no_shuffle, LanguageVoice::Spanisch)?,
        ReviewCmd::WorteDe {
            section,
            batch,
            no_shuffle,
        } => worte::run(config, section, batch, no_shuffle, LanguageVoice::Deutsch)?,
        ReviewCmd::Setze {
            section,
            batch,
            no_shuffle,
        } => setze::run(config, section, batch, no_shuffle)?,
    };

    Ok(())
}
