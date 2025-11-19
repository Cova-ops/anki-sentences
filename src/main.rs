use color_eyre::eyre::Result;

use crate::{console::menu_main::menu_main, db::init_db};

mod console;
mod db;
mod helpers;
mod utils;

fn main() -> Result<()> {
    color_eyre::install().unwrap();
    run()
}

fn run() -> Result<()> {
    init_db()?;
    menu_main()?;
    Ok(())
}
