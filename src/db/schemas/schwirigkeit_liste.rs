use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use chrono::{DateTime, Utc};
use color_eyre::eyre::{Result, eyre};

pub const CREATE_STR_TABLE_SCHWIRIGKEIT_LISTE: &str = "
CREATE TABLE IF NOT EXISTS schwirigkeit_liste (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    schwirigkeit        TEXT,
    created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at          TEXT
)";

static HASH_VALUES: LazyLock<Mutex<HashMap<i32, SchwirigkeitListeSchema>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

// 0: einfag
// 1: normal
// 2: schwirig
#[derive(Debug, Clone)]
pub struct SchwirigkeitListeSchema {
    pub id: i32,
    pub schwirigkeit: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl SchwirigkeitListeSchema {
    pub fn init_data(data: &[SchwirigkeitListeSchema]) -> Result<()> {
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
            .ok_or_else(|| eyre!("[SchwirigkeitListe.from_id] id no encontrado: {}", id))
    }

    pub fn from_name(name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        let hash = HASH_VALUES.lock().unwrap();
        hash.iter()
            .find(|(_, value)| value.schwirigkeit == name)
            .map(|(_, value)| Self { ..value.clone() })
            .ok_or_else(|| eyre!("[SchwirigkeitListe.from_name] name no encontrado: {}", name))
    }
}

// 0: einfag
// 1: normal
// 2: schwirig
#[derive(Debug, Clone)]
pub struct NewSchwirigkeitListeSchema {
    pub id: i32,
    pub schwirigkeit: String,
}

impl NewSchwirigkeitListeSchema {
    pub fn new(id: impl Into<i32>, schwirigkeit: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schwirigkeit: schwirigkeit.into(),
        }
    }
}
