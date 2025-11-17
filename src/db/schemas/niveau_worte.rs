use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use chrono::{DateTime, Utc};
use color_eyre::eyre::{Result, eyre};

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

static HASH_VALUES: LazyLock<Mutex<HashMap<i32, NiveauWorteSchema>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

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

impl NiveauWorteSchema {
    pub fn init_data(data: &[Self]) -> Result<()> {
        let mut hash = HASH_VALUES.lock().unwrap();
        for d in data {
            hash.insert(d.id, d.clone());
        }
        Ok(())
    }

    pub fn from_id(id: impl Into<i32>) -> Result<Self> {
        let id = id.into();
        let hash = HASH_VALUES.lock().unwrap();
        hash.get(&id)
            .cloned()
            .ok_or_else(|| eyre!("[NiveauWorteSchema.from_id] id no encontrado: {}", id))
    }
}

#[derive(Debug, Clone)]
pub struct NewNiveauWorteSchema {
    pub id: i32,
    pub niveau: String,
}

impl NewNiveauWorteSchema {
    pub fn new<S>(id: impl Into<i32>, gender: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            id: id.into(),
            niveau: gender.into(),
        }
    }
}
