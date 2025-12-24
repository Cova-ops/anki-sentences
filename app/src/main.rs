use color_eyre::eyre::Result;
use dotenvy;

use crate::helpers::toml::AppConfig;

mod commands;
mod console;
mod db;
mod helpers;
mod services;
mod traits;
mod utils;

fn main() -> Result<()> {
    dotenvy::dotenv()?;
    color_eyre::install().unwrap();
    run()
}

fn run() -> Result<()> {
    let mut config = AppConfig::load_config()?;
    let name_db = config.get_database_path()?;

    db::init_db(name_db)?;
    console::menu_main(&mut config)?;

    Ok(())
}
