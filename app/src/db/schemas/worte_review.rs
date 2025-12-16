use chrono::{DateTime, Utc};
use sql_model::SqlModel;

pub const CREATE_STR_TABLE_WORTE_REVIEW: &str = "
CREATE TABLE IF NOT EXISTS worte_review (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    wort_id         INTEGER NOT NULL UNIQUE,
    interval        INTEGER NOT NULL,
    ease_factor     REAL    NOT NULL,
    repetitions     INTEGER NOT NULL,
    last_review     TEXT NOT NULL,
    next_review     TEXT NOT NULL,
    created_at      TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TEXT,
    FOREIGN KEY(wort_id) REFERENCES worte(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
";

pub const CREATE_STR_INDEX_WORTE_REVIEW: &str = "
    CREATE UNIQUE INDEX IF NOT EXISTS idx_worte_review_wort_id ON worte_review(wort_id);
    CREATE INDEX IF NOT EXISTS idx_worte_review_next_review ON worte_review(next_review);
";

#[derive(Debug, Clone)]
pub struct WorteReviewSchema {
    pub id: i32,

    pub wort_id: i32,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: DateTime<Utc>,
    pub next_review: DateTime<Utc>,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, SqlModel)]
#[sql(insert(wort_id, interval, ease_factor, repetitions, last_review, next_review))]
pub struct NewWorteReviewSchema {
    pub wort_id: i32,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: String, // DateTime<Utc>
    pub next_review: String, // DateTime<Utc>
}

#[derive(Debug, SqlModel)]
#[sql(raw(
    id,
    wort_id,
    interval,
    ease_factor,
    repetitions,
    last_review,
    next_review,
    created_at,
    deleted_at
))]
pub struct RawWorteReviewSchema {
    pub id: i32,
    pub wort_id: i32,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: String,
    pub next_review: String,

    // Generic
    pub created_at: String,
    pub deleted_at: Option<String>,
}
