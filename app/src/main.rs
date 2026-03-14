use dotenvy;

use crate::helpers::{error_handler::AppError, toml::AppConfig};

mod commands;
mod console;
mod db;
mod helpers;
mod services;
mod traits;
mod utils;

#[cfg(test)]
mod test_utils;

fn main() -> Result<(), AppError> {
    dotenvy::dotenv()?;
    run()
}

fn run() -> Result<(), AppError> {
    let mut config = AppConfig::load_config()?;

    helpers::audios::ManageAudios::new(
        config.get_path_audios_worte()?,
        config.get_path_audios_setze()?,
        config.get_path_audios_artikel()?,
    )
    .check_audios_artikel()?;

    let mut conn = db::get_conn(config.get_database_path()?)?;
    db::init_db(&mut conn)?;
    console::menu_main(&mut config)?;

    Ok(())
}
