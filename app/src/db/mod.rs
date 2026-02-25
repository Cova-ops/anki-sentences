mod connection;
mod repositories;
pub mod schemas;
pub mod seeders;
pub mod traits;
pub mod views;

use std::path::Path;

pub use connection::*;
pub use repositories::*;

use crate::{
    db::{schemas::init_schemas, seeders::init_data},
    helpers::error_handler::AppError,
};

pub fn init_db(name_db: &Path) -> Result<(), AppError> {
    let mut conn = get_conn(name_db)?;
    init_schemas(&mut conn)?;
    init_data(&mut conn)?;
    Ok(())
}
