mod connection;
mod repositories;
pub mod schemas;
mod seeders;

use color_eyre::eyre::Result;

pub use connection::*;
pub use repositories::*;
pub use seeders::*;

pub fn init_db() -> Result<()> {
    init_schemas()?;
    init_seeds()?;
    init_data();
    Ok(())
}
