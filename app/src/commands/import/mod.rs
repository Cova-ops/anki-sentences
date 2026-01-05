use color_eyre::Result;

use crate::{
    console::cli::{ExportImportCmd, ReviewCmd},
    helpers::toml::AppConfig,
};

// mod setze;
mod worte;

pub fn run(cmd: ExportImportCmd, config: &AppConfig) -> Result<()> {
    match cmd {
        ExportImportCmd::Worte { path, type_file } => worte::run(config, path, type_file)?,
        _ => todo!("Aguantame papito"),
    };

    Ok(())
}
