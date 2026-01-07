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

#[cfg(test)]
mod test_utils;

fn main() -> Result<()> {
    dotenvy::dotenv()?;
    color_eyre::install().unwrap();
    run()
}

fn run() -> Result<()> {
    let mut config = AppConfig::load_config()?;
    let name_db = config.get_database_path()?;

    helpers::audios::ManageAudios::new(
        config.get_path_audios_worte()?,
        config.get_path_audios_setze()?,
        config.get_path_audios_artikel()?,
    )
    .check_audios_artikel()?;

    db::init_db(name_db)?;
    console::menu_main(&mut config)?;

    Ok(())
}
