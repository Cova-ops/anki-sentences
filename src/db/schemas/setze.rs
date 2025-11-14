use chrono::{DateTime, Utc};

use crate::db::schemas::schwirigkeit_liste::SchwirigkeitListeSchema;

pub const CREATE_STR_TABLE_SETZE: &str = "
CREATE TABLE IF NOT EXISTS setze (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    setze_spanisch      TEXT NOT NULL,
    setze_deutsch       TEXT NOT NULL,
    thema               TEXT NOT NULL,
    schwirigkeit_id     INTEGER NOT NULL,
    created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at          TEXT,
    FOREIGN KEY(schwirigkeit_id) REFERENCES schwirigkeit_liste(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)";

pub const CREATE_STR_INDEX_SETZE: &str = "
CREATE INDEX IF NOT EXISTS idx_setze_setze_spanisch ON setze(setze_spanisch);
CREATE INDEX IF NOT EXISTS idx_setze_setze_deutsch ON setze(setze_deutsch);
CREATE INDEX IF NOT EXISTS idx_setze_thema ON setze(thema);
CREATE INDEX IF NOT EXISTS idx_setze_schwirigkeit_id ON setze(schwirigkeit_id);
";

#[derive(Debug, Clone)]
pub struct SetzeSchema {
    pub id: i32,
    pub setze_spanisch: String,
    pub setze_deutsch: String,
    pub schwirig_id: SchwirigkeitListeSchema,
    pub thema: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct NewSetzeSchema {
    pub setze_spanisch: String,
    pub setze_deutsch: String,
    pub schwirig_id: i32,
    pub thema: String,
}

impl NewSetzeSchema {
    pub fn new(
        setze_spanisch: impl Into<String>,
        setze_deutsch: impl Into<String>,
        thema: impl Into<String>,
        schwirig_id: impl Into<i32>,
    ) -> Self {
        Self {
            setze_spanisch: setze_spanisch.into(),
            setze_deutsch: setze_deutsch.into(),
            schwirig_id: schwirig_id.into(),
            thema: thema.into(),
        }
    }
}
