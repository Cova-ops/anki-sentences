mod connection;
mod repositories;
mod schemas;
mod seeders;

use color_eyre::eyre::Result;

pub use connection::*;
pub use repositories::*;
pub use schemas::*;
pub use seeders::*;

pub fn init_db() -> Result<()> {
    init_schemas()?;
    init_seeds()?;
    init_data();
    Ok(())
}
