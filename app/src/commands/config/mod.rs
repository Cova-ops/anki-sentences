use color_eyre::Result;

use crate::{console::cli::ConfigCmd, helpers::toml::AppConfig};

mod clean_data;
mod download_audios;

pub fn run(cmd: ConfigCmd, config: &AppConfig) -> Result<()> {
    match cmd {
        ConfigCmd::CleanData {} => clean_data::run(config)?,
        ConfigCmd::DownloadAudios {} => download_audios::run(config)?,
        _ => todo!("Aguantame papito"),
    };

    Ok(())
}
