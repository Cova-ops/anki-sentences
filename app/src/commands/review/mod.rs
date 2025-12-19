use color_eyre::Result;

use crate::{console::cli::ReviewCmd, helpers::toml::AppConfig};

mod setze;
mod worte;

pub fn run(cmd: ReviewCmd, config: &AppConfig) -> Result<()> {
    match cmd {
        ReviewCmd::Worte {
            section,
            batch,
            no_shuffle,
        } => worte::run(config, section, batch, no_shuffle)?,
        ReviewCmd::Setze {
            section,
            batch,
            no_shuffle,
        } => setze::run(config, section, batch, no_shuffle)?,
    };

    Ok(())
}
