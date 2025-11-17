use chrono::{DateTime, Utc};

use crate::db::schemas::{gender_worte::GenderWorteSchema, niveau_worte::NiveauWorteSchema};

pub const CREATE_STR_TABLE_WORTE: &str = "
CREATE TABLE IF NOT EXISTS worte(
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    gender_id           INTEGER,                            -- table gender_worte
    wort_de             TEXT NOT NULL,                   
    wort_es             TEXT NOT NULL,                   
    plural              TEXT,
    niveau_id           INTEGER NOT NULL,                   -- table niveau_worte
    example_de          TEXT,                   
    example_es          TEXT,                   

    -- nur verben
    verb_aux TEXT,              -- 'sein' / 'haben' / NULL
    trennbar BOOLEAN,           -- verbo separable
    reflexiv BOOLEAN,

    -- Generic
    created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at          TEXT,

    FOREIGN KEY(gender_id) REFERENCES gender_worte(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    

    FOREIGN KEY(niveau_id) REFERENCES niveau_worte(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)";

pub const CREATE_STR_INDEX_WORTE: &str = "
    CREATE INDEX IF NOT EXISTS idx_worte_created_at ON worte(created_at);
    CREATE INDEX IF NOT EXISTS idx_worte_gender_id ON worte(gender_id);
    CREATE INDEX IF NOT EXISTS idx_worte_niveau_id ON worte(niveau_id);
";

#[derive(Debug, Clone)]
pub struct WorteSchema {
    pub id: i32,
    pub gender_id: Option<GenderWorteSchema>,
    pub worte_de: String,
    pub worte_es: String,
    pub plural: Option<String>,
    pub niveau_id: Option<NiveauWorteSchema>,
    pub example_de: Option<String>,
    pub example_es: Option<String>,

    // nur verben
    pub verb_aux: Option<String>,
    pub trennbar: Option<bool>,
    pub reflexiv: Option<bool>,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
