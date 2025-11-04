use color_eyre::eyre::{Result, eyre};

use crate::{console::menu_main, db::init_db};

mod console;
mod db;
mod helpers;
mod utils;
fn main() -> Result<()> {
    color_eyre::install().unwrap();

    // return Err(eyre!("Error de pruebas"));

    run()
}

fn run() -> Result<()> {
    init_db()?;
    menu_main()?;
    Ok(())
}
