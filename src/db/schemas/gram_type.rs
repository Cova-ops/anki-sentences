use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use chrono::{DateTime, Utc};
use color_eyre::eyre::{Result, eyre};

pub const CREATE_STR_TABLE_GRAM_TYPE: &str = "
CREATE TABLE IF NOT EXISTS gram_type(
    id INTEGER PRIMARY KEY,
    code TEXT UNIQUE NOT NULL,
    name TEXT NOT NULL,

    -- Generic
    created_at TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at TEXT
);";

pub const CREATE_STR_INDEX_GRAM_TYPE: &str = "
    CREATE INDEX IF NOT EXISTS idx_gram_type_code ON gram_type(code)
";

static HASH_VALUES: LazyLock<Mutex<HashMap<i32, GramTypeSchema>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone)]
pub struct GramTypeSchema {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl GramTypeSchema {
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
pub struct NewGramTypeSchema {
    pub id: i32,
    pub code: String,
    pub name: String,
}

impl NewGramTypeSchema {
    pub fn new<I, S>(id: I, code: S, name: S) -> Self
    where
        I: Into<i32>,
        S: Into<String>,
    {
        Self {
            id: id.into(),
            code: code.into(),
            name: name.into(),
        }
    }
}
