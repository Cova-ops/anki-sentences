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

pub const CREATE_STR_TABLE_WORTE_REVIEW: &str = "
CREATE TABLE IF NOT EXISTS worte_review (
    wort_id         INTEGER NOT NULL,
    direction       TEXT NOT NULL,
    interval        INTEGER NOT NULL,
    ease_factor     REAL    NOT NULL,
    repetitions     INTEGER NOT NULL,
    last_review     TEXT NOT NULL,
    next_review     TEXT NOT NULL,
    created_at      TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TEXT,

    PRIMARY KEY(wort_id, direction),

    FOREIGN KEY(wort_id) REFERENCES worte(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
";

pub const CREATE_STR_INDEX_WORTE_REVIEW: &str = "
    CREATE INDEX IF NOT EXISTS idx_worte_review_wort_id ON worte_review(wort_id);
    CREATE INDEX IF NOT EXISTS idx_worte_review_next_review ON worte_review(next_review);
";
