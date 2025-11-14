use chrono::{DateTime, Utc};

pub const CREATE_STR_TABLE_GESCHICHTLICH_SETZE: &str = "
CREATE TABLE IF NOT EXISTS  (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    setze_id            INTEGER NOT NULL,
    result              BOOL NOT NULL,
    created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at          TEXT,
    FOREIGN KEY(setze_id) REFERENCES setze(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)";

pub const CREATE_STR_INDEX_GESCHICHTLICH_SETZE: &str = "
CREATE INDEX IF NOT EXISTS idx_geschichtlich_setze_created_at ON geschichtlich_setze(created_at);
";

#[derive(Debug, Clone)]
pub struct GeschichtlichSetzeSchema {
    pub id: i32,
    pub setze_id: i32,
    pub result: bool, // 0: schlecht, 1: gut
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
