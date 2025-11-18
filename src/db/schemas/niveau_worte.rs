use chrono::{DateTime, Utc};

pub const CREATE_STR_TABLE_GENDER_WORTE: &str = "
    CREATE TABLE IF NOT EXISTS niveau_worte (
        id                  INTEGER PRIMARY KEY AUTOINCREMENT,
        niveau              TEXT NOT NULL,                         

        -- Generic
        created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
        deleted_at          TEXT
    )
";

pub const CREATE_STR_INDEX_NIVEAU_WORTE: &str = "
    CREATE INDEX IF NOT EXISTS idx_niveau_worte_created_at ON niveau_worte(created_at);
";

// 0 - A1
// 1 - A2
// 2 - B1
// 3 - B2
// 4 - C1
// 5 - C2
#[derive(Debug, Clone)]
pub struct NiveauWorteSchema {
    pub id: i32,
    pub niveau: String,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct NewNiveauWorteSchema {
    pub id: i32,
    pub niveau: String,
}

#[derive(Debug)]
pub struct RawNiveauWorteSchema {
    pub id: i32,
    pub niveau: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
}
