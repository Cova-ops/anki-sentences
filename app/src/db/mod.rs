mod connection;
pub mod raw;
mod repositories;
pub mod schemas;
pub mod seeders;
pub mod traits;

use color_eyre::eyre::Result;

pub use connection::*;
pub use repositories::*;

use crate::db::{schemas::init_schemas, seeders::init_data};

pub fn init_db() -> Result<()> {
    let mut conn = get_conn();
    init_schemas(&mut conn)?;
    init_data(&mut conn)?;
    Ok(())
}
