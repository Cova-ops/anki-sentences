mod input;
mod model;
mod schema;
mod snapshot;

pub use input::*;
pub use model::*;
pub use snapshot::*;

pub const CREATE_STR_TABLE_WORTE_TYPE_GRAM: &str = "
CREATE TABLE IF NOT EXISTS worte_gram_type(
    id_gram_type        INTEGER NOT NULL,

    -- Generic
    created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at          TEXT,

    PRIMARY KEY(id_worte,id_gram_type),

    FOREIGN KEY(id_worte) REFERENCES worte(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    FOREIGN KEY(id_gram_type) REFERENCES gram_type(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)";

pub const CREATE_STR_INDEX_WORTE_TYPE_GRAM: &str = "
    CREATE INDEX IF NOT EXISTS idx_worte_gram_type_id_worte ON worte_gram_type(id_worte);
    CREATE INDEX IF NOT EXISTS idx_worte_gram_type_id_gram_type ON worte_gram_type(id_gram_type);
";
