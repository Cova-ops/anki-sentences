use color_eyre::eyre::Result;

use crate::helpers::audios::ManageAudios;

mod console;
mod db;
mod helpers;
mod services;
mod traits;
mod utils;

fn main() -> Result<()> {
    color_eyre::install().unwrap();
    run()
}

fn run() -> Result<()> {
    db::init_db()?;
    ManageAudios::init_dir()?;
    console::menu_main()?;
    Ok(())
}
