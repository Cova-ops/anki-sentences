mod input;
mod model;
mod schema;
mod snapshot;

pub use input::*;
pub use model::*;
pub use schema::*;
pub use snapshot::*;

pub const CREATE_STR_TABLE_SETZE_REVIEW: &str = "
CREATE TABLE IF NOT EXISTS setze_review (
    satz_id         INTEGER NOT NULL,
    direction       TEXT NOT NULL,
    interval        INTEGER NOT NULL,
    ease_factor     REAL    NOT NULL,
    repetitions     INTEGER NOT NULL,
    last_review     TEXT NOT NULL,
    next_review     TEXT NOT NULL,

    created_at      TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TEXT,

    PRIMARY KEY(satz_id, direction),

    FOREIGN KEY(satz_id) REFERENCES setze(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
";

pub const CREATE_STR_INDEX_SETZE_REVIEW: &str = "
    CREATE INDEX IF NOT EXISTS idx_setze_review_satz_id ON setze_review(satz_id);
    CREATE INDEX IF NOT EXISTS idx_setze_review_next_review ON setze_review(next_review);
";
