use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use chrono::{DateTime, Utc};
use color_eyre::eyre::{Result, eyre};

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

static HASH_VALUES: LazyLock<Mutex<HashMap<i32, GenderWorteSchema>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

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

impl GenderWorteSchema {
    pub fn init_data(data: &[GenderWorteSchema]) -> Result<()> {
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
            .ok_or_else(|| eyre!("[GenderWorteSchema.from_id] id no encontrado: {}", id))
    }
}

#[derive(Debug, Clone)]
pub struct NewGenderWorteSchema {
    pub id: i32,
    pub gender: String,
    pub artikel: String,
}

impl NewGenderWorteSchema {
    pub fn new<S>(id: impl Into<i32>, gender: S, artikel: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            id: id.into(),
            gender: gender.into(),
            artikel: artikel.into(),
        }
    }
}
