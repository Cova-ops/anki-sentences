use chrono::{DateTime, Utc};
use sql_model::SqlModel;

pub const CREATE_STR_TABLE_GESCHICHTLICH_SETZE: &str = "
CREATE TABLE IF NOT EXISTS geschichtlich_setze (
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

#[derive(Debug, Clone, SqlModel)]
#[sql(raw_type = "RawGeschichtlichSetzeSchema")]
#[sql(raw(id, setze_id, result, created_at, deleted_at))]
pub struct GeschichtlichSetzeSchema {
    pub id: i32,
    pub setze_id: i32,
    pub result: bool, // 0: schlecht, 1: gut
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, SqlModel)]
#[sql(insert(setze_id, result))]
pub struct NewGeschichtlichSetzeSchema {
    pub setze_id: i32,
    pub result: bool,
}

#[derive(Debug, SqlModel)]
#[sql(raw(id, setze_id, result, created_at, deleted_at))]
pub struct RawGeschichtlichSetzeSchema {
    pub id: i32,
    pub setze_id: i32,
    pub result: bool,
    pub created_at: String,
    pub deleted_at: Option<String>,
}
