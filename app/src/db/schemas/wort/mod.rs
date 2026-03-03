mod input;
mod model;
mod schema;
mod snapshot;

pub use input::*;
pub use model::*;
pub use schema::*;

#[allow(unused_imports)]
pub use snapshot::*;

pub const CREATE_STR_TABLE_WORTE: &str = "
CREATE TABLE IF NOT EXISTS worte(
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    gender_id           INTEGER,                            -- table worte_gender
    wort_de             TEXT NOT NULL,                   
    wort_es             TEXT NOT NULL,                   
    plural              TEXT,
    niveau_id           INTEGER NOT NULL,                   -- table niveau_liste
    example_de          TEXT,                   
    example_es          TEXT,                   

    -- nur verben
    verb_aux TEXT,              -- 'sein' / 'haben' / NULL
    trennbar BOOLEAN,           -- verbo separable
    reflexiv BOOLEAN,

    -- Generic
    created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at          TEXT,

    FOREIGN KEY(gender_id) REFERENCES worte_gender(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,    

    FOREIGN KEY(niveau_id) REFERENCES niveau_liste(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)";

pub const CREATE_STR_INDEX_WORTE: &str = "
    CREATE INDEX IF NOT EXISTS idx_worte_created_at ON worte(created_at);
    CREATE INDEX IF NOT EXISTS idx_worte_gender_id ON worte(gender_id);
    CREATE INDEX IF NOT EXISTS idx_worte_niveau_id ON worte(niveau_id);
    CREATE UNIQUE INDEX IF NOT EXISTS ux_worte_exact_es_de ON worte(wort_es, wort_de) WHERE deleted_at IS NULL;
";
