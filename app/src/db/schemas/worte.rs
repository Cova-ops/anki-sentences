use chrono::{DateTime, Utc};
use sql_model::SqlModel;

use crate::db::schemas::{
    gender_worte::GenderWorteSchema, gram_type::GramTypeSchema, niveau_worte::NiveauWorteSchema,
};

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
        ON UPDATE CASCADE,    

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
    pub gram_type_id: Vec<GramTypeSchema>,
    pub gender_id: Option<GenderWorteSchema>,
    pub worte_de: String,
    pub worte_es: String,
    pub plural: Option<String>,
    pub niveau_id: NiveauWorteSchema,
    pub example_de: String,
    pub example_es: String,

    // nur verben
    pub verb_aux: Option<String>,
    pub trennbar: Option<bool>,
    pub reflexiv: Option<bool>,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, SqlModel)]
#[sql(insert(
    gender_id, worte_de, worte_es, plural, niveau_id, example_de, example_es, verb_aux, trennbar,
    reflexiv
))]
pub struct NewWorteSchema {
    pub gram_type: Vec<i32>,
    pub gender_id: Option<i32>,
    pub worte_de: String,
    pub worte_es: String,
    pub plural: Option<String>,
    pub niveau_id: i32,
    pub example_de: String,
    pub example_es: String,

    // nur verben
    pub verb_aux: Option<String>,
    pub trennbar: Option<bool>,
    pub reflexiv: Option<bool>,
}

#[derive(Debug, SqlModel)]
#[sql(raw(
    id, gender_id, worte_de, worte_es, plural, niveau_id, example_de, example_es, verb_aux,
    trennbar, reflexiv, created_at, deleted_at
))]
pub struct RawWorteSchema {
    pub id: i32,
    pub gender_id: Option<i32>,
    pub worte_de: String,
    pub worte_es: String,
    pub plural: Option<String>,
    pub niveau_id: i32,
    pub example_de: String,
    pub example_es: String,

    // nur verben
    pub verb_aux: Option<String>,
    pub trennbar: Option<bool>,
    pub reflexiv: Option<bool>,

    // Generic
    pub created_at: String,
    pub deleted_at: Option<String>,
}
