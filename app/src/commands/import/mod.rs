use crate::{
    console::cli::ExportImportCmd,
    helpers::{error_handler::AppError, toml::AppConfig},
};

// mod setze;
mod wort;

pub fn run(cmd: ExportImportCmd, config: &AppConfig) -> Result<(), AppError> {
    match cmd {
        ExportImportCmd::Worte { path, type_file } => wort::run(config, &path, type_file)?,
        _ => todo!("Aguantame papito"),
    };

    Ok(())
}
