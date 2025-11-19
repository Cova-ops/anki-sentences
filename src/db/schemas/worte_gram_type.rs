use chrono::{DateTime, Utc};

pub const CREATE_STR_TABLE_WORTE_TYPE_GRAM: &str = "
CREATE TABLE IF NOT EXISTS worte_type_gram(
    id_worte            INTEGER NOT NULL,
    id_gram_type        INTEGER NOT NULL,

    -- Generic
    created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at          TEXT,

    PRIMARY KEY(id_worte,id_gram_type),

    FOREIGN KEY(id_worte) REFERENCES wort(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    FOREIGN KEY(id_gram_type) REFERENCES gram_type(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)";

pub const CREATE_STR_INDEX_WORTE_TYPE_GRAM: &str = "
    CREATE INDEX IF NOT EXISTS idx_worte_gram_type_id_worte ON worte_type_gram(id_worte);
    CREATE INDEX IF NOT EXISTS idx_worte_gram_type_id_gram_type ON worte_type_gram(id_gram_type);
";

#[derive(Debug, Clone)]
pub struct WorteGramTypeSchema {
    pub id_worte: i32,
    pub id_gram_type: i32,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct NewWorteGramTypeSchema {
    pub id_worte: i32,
    pub id_gram_type: i32,
}

#[derive(Debug)]
pub struct RawWorteGramTypeSchema {
    pub id_worte: i32,
    pub id_gram_type: i32,
    pub created_at: String,
    pub deleted_at: Option<String>,
}
