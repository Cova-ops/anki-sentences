mod enums;
mod input;
mod model;
mod schema;
mod snapshot;

pub use enums::*;
pub use input::*;
pub use model::*;
pub use schema::*;

#[allow(unused_imports)]
pub use snapshot::*;

pub const CREATE_STR_TABLE_NIVEAU_LISTE: &str = "
    CREATE TABLE IF NOT EXISTS niveau_liste (
        id                  INTEGER PRIMARY KEY AUTOINCREMENT,
        niveau              TEXT NOT NULL,                         

        -- Generic
        created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
        deleted_at          TEXT
    )
";

pub const CREATE_STR_INDEX_NIVEAU_LISTE: &str = "
    CREATE INDEX IF NOT EXISTS idx_niveau_liste_created_at ON niveau_liste(created_at);
";
