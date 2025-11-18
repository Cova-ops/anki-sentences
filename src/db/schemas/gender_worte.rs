use chrono::{DateTime, Utc};

pub const CREATE_STR_TABLE_GENDER_WORTE: &str = "
    CREATE TABLE IF NOT EXISTS gender_worte (
        id                  INTEGER PRIMARY KEY AUTOINCREMENT,
        gender              TEXT NOT NULL,                         
        artikel             TEXT NOT NULL,

        -- Generic
        created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
        deleted_at          TEXT
    )
";

pub const CREATE_STR_INDEX_GENDER_WORTE: &str = "
    CREATE INDEX IF NOT EXISTS idx_gender_worte_created_at ON gender_worte(created_at);
";

// 0 - Maskuline - der
// 1 - Femenin - die
// 2 - Neutrum - das
// 3 - Plural - die
#[derive(Debug, Clone)]
pub struct GenderWorteSchema {
    pub id: i32,
    pub gender: String,
    pub artikel: String,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct NewGenderWorteSchema {
    pub id: i32,
    pub gender: String,
    pub artikel: String,
}

#[derive(Debug)]
pub struct RawGenderWorteSchema {
    pub id: i32,
    pub gender: String,
    pub artikel: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
}
