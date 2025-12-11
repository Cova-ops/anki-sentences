use chrono::{DateTime, Utc};
use sql_model::SqlModel;

pub const CREATE_STR_TABLE_NIVEAU_LISTE: &str = "
    CREATE TABLE IF NOT EXISTS niveau_liste (
        id                  INTEGER PRIMARY KEY AUTOINCREMENT,
        niveau              TEXT NOT NULL,                         

        -- Generic
        created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
        deleted_at          TEXT
    )
";

pub const CREATE_STR_INDEX_NIVEAU_LISTE: &str = "
    CREATE INDEX IF NOT EXISTS idx_niveau_liste_created_at ON niveau_liste(created_at);
";

// 0 - A1
// 1 - A2
// 2 - B1
// 3 - B2
// 4 - C1
// 5 - C2
#[derive(Debug, Clone, SqlModel)]
#[sql(raw_type = "RawNiveauListeSchema")]
#[sql(raw(id, niveau, created_at, deleted_at))]
pub struct NiveauListeSchema {
    pub id: i32,
    pub niveau: String,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, SqlModel)]
#[sql(insert(id, niveau))]
pub struct NewNiveauListeSchema {
    pub id: i32,
    pub niveau: String,
}

#[derive(Debug, SqlModel)]
#[sql(raw(id, niveau, created_at, deleted_at))]
pub struct RawNiveauListeSchema {
    pub id: i32,
    pub niveau: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
}
