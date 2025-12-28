use color_eyre::Result;

use crate::{
    console::cli::{ConfigCmd, ExportImportCmd, ReviewCmd},
    helpers::toml::AppConfig,
};

mod clean_data;

pub fn run(cmd: ConfigCmd, config: &AppConfig) -> Result<()> {
    match cmd {
        ConfigCmd::CleanData {} => clean_data::run(config)?,
        _ => todo!("Aguantame papito"),
    };

    Ok(())
}
