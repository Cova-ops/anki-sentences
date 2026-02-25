use crate::{
    console::cli::ConfigCmd,
    helpers::{error_handler::AppError, toml::AppConfig},
};

mod clean_data;
mod download_audios;

pub fn run(cmd: ConfigCmd, config: &AppConfig) -> Result<(), AppError> {
    match cmd {
        ConfigCmd::CleanData {} => clean_data::run(config)?,
        ConfigCmd::DownloadAudios {} => download_audios::run(config)?,
        _ => todo!("Aguantame papito"),
    };

    Ok(())
}
