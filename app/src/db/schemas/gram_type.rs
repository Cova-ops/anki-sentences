use chrono::{DateTime, Utc};
use sql_model::SqlModel;

pub const CREATE_STR_TABLE_GRAM_TYPE: &str = "
CREATE TABLE IF NOT EXISTS gram_type(
    id INTEGER PRIMARY KEY,
    code TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at TEXT
);";

pub const CREATE_STR_INDEX_GRAM_TYPE: &str = "
    CREATE INDEX IF NOT EXISTS idx_gram_type_code ON gram_type(code)
";

#[derive(Debug, Clone, SqlModel)]
#[sql(raw_type = "RawGramTypeSchema")]
#[sql(raw(id, code, name, created_at, deleted_at))]
pub struct GramTypeSchema {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, SqlModel)]
#[sql(insert(id, code, name))]
pub struct NewGramTypeSchema {
    pub id: i32,
    pub code: String,
    pub name: String,
}

#[derive(Debug, SqlModel)]
#[sql(raw(id, code, name, created_at, deleted_at))]
pub struct RawGramTypeSchema {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
}
