mod input;
mod model;
mod schema;
mod snapshot;

pub use input::*;
pub use model::*;
pub use schema::*;
pub use snapshot::*;

pub const CREATE_STR_TABLE_SETZE: &str = "
CREATE TABLE IF NOT EXISTS setze (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    setze_spanisch      TEXT NOT NULL,
    setze_deutsch       TEXT NOT NULL,
    thema               TEXT NOT NULL,
    niveau_id           INTEGER NOT NULL,
    created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at          TEXT,

    FOREIGN KEY(niveau_id) REFERENCES niveau_liste(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)";

pub const CREATE_STR_INDEX_SETZE: &str = "
    CREATE INDEX IF NOT EXISTS idx_setze_setze_spanisch ON setze(setze_spanisch);
    CREATE INDEX IF NOT EXISTS idx_setze_setze_deutsch ON setze(setze_deutsch);
    CREATE INDEX IF NOT EXISTS idx_setze_thema ON setze(thema);
    CREATE INDEX IF NOT EXISTS idx_setze_niveau_id ON setze(niveau_id);
";
