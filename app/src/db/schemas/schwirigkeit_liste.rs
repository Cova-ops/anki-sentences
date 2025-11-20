use chrono::{DateTime, Utc};
use sql_model::SqlModel;

pub const CREATE_STR_TABLE_SCHWIRIGKEIT_LISTE: &str = "
CREATE TABLE IF NOT EXISTS schwirigkeit_liste (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    schwirigkeit        TEXT,
    created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at          TEXT
)";

// 0: einfag
// 1: normal
// 2: schwirig
#[derive(Debug, Clone, SqlModel)]
#[sql(raw_type = "RawSchwirigkeitListeSchema")]
#[sql(raw(id, schwirigkeit, created_at, deleted_at))]
pub struct SchwirigkeitListeSchema {
    pub id: i32,
    pub schwirigkeit: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

// 0: einfag
// 1: normal
// 2: schwirig
#[derive(Debug, Clone, SqlModel)]
#[sql(insert(id, schwirigkeit))]
pub struct NewSchwirigkeitListeSchema {
    pub id: i32,
    pub schwirigkeit: String,
}

#[derive(Debug, SqlModel)]
#[sql(raw(id, schwirigkeit, created_at, deleted_at))]
pub struct RawSchwirigkeitListeSchema {
    pub id: i32,
    pub schwirigkeit: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
}
