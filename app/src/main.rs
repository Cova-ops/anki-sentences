use color_eyre::eyre::Result;

use crate::{console::menu_main::menu_main, db::init_db, helpers::audios::ManageAudios};

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
    init_db()?;
    ManageAudios::init_dir()?;
    menu_main()?;
    Ok(())
}
