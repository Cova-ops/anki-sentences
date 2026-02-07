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

pub const CREATE_STR_TABLE_GRAM_TYPE: &str = "
CREATE TABLE IF NOT EXISTS gram_type(
    id              INTEGER PRIMARY KEY,
    code            TEXT UNIQUE NOT NULL,
    name            TEXT NOT NULL,
    created_at      TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TEXT
);";

pub const CREATE_STR_INDEX_GRAM_TYPE: &str = "
    CREATE INDEX IF NOT EXISTS idx_gram_type_code ON gram_type(code)
";
