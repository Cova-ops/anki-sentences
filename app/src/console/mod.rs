use clap::Parser;
use color_eyre::eyre::Result;

use crate::{
    commands,
    console::cli::{Cli, Command},
    helpers::toml::AppConfig,
};

pub mod cli;

pub fn menu_main(config: &mut AppConfig) -> Result<()> {
    // clean_screen();

    let cli = Cli::parse();

    match cli.cmd {
        Command::Review { cmd } => commands::review::run(cmd, config)?,
        Command::Import { cmd } => commands::import::run(cmd, config)?,
        Command::Config { cmd } => commands::config::run(cmd, config)?,
        _ => todo!("Todavia no papito"),
    };

    Ok(())
}
