use chrono::{DateTime, Utc};

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

#[derive(Debug, Clone)]
pub struct GramTypeSchema {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct NewGramTypeSchema {
    pub id: i32,
    pub code: String,
    pub name: String,
}

#[derive(Debug)]
pub struct RawGramTypeSchema {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
}
