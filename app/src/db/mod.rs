mod connection;
mod repositories;
pub mod schemas;
pub mod seeders;
pub mod traits;
pub mod views;

pub use connection::*;
pub use repositories::*;

use crate::{
    db::{schemas::init_schemas, seeders::init_data},
    helpers::error_handler::AppError,
};

pub fn init_db(conn: &mut rusqlite::Connection) -> Result<(), AppError> {
    init_schemas(conn)?;
    init_data(conn)?;
    Ok(())
}
