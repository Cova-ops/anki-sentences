mod enums;
mod input;
mod model;
mod schema;
mod snapshot;

pub use enums::*;
pub use input::*;
pub use model::*;
pub use schema::*;
pub use snapshot::*;

pub const CREATE_STR_TABLE_WORTE_GENDER: &str = "
    CREATE TABLE IF NOT EXISTS worte_gender (
        id                  INTEGER PRIMARY KEY AUTOINCREMENT,
        gender              TEXT NOT NULL,                         
        artikel             TEXT NOT NULL,

        -- Generic
        created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
        deleted_at          TEXT
    )
";

pub const CREATE_STR_INDEX_WORTE_GENDER: &str = "
    CREATE INDEX IF NOT EXISTS idx_worte_gender_created_at ON worte_gender(created_at);
";
