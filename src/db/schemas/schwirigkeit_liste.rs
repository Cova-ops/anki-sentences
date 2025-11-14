use chrono::{DateTime, Utc};
use color_eyre::eyre::{Result, eyre};

use crate::db::SchwirigkeitListeRepo;

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
#[derive(Debug, Clone)]
pub struct SchwirigkeitListeSchema {
    pub id: i32,
    pub schwirigkeit: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl SchwirigkeitListeSchema {
    pub fn init_data() {
        SchwirigkeitListeRepo::fetch_all();
    }

    pub fn from_id(id: impl Into<i32>) -> Result<Self> {
        let id = id.into();
        SchwirigkeitListeRepo::fetch_all()
            .get(&id)
            .cloned()
            .ok_or_else(|| eyre!("[SchwirigkeitListe.from_id] id no encontrado: {}", id))
    }

    pub fn from_name(name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        SchwirigkeitListeRepo::fetch_all()
            .iter()
            .find(|(_, value)| value.schwirigkeit == name)
            .map(|(_, value)| Self { ..value.clone() })
            .ok_or_else(|| eyre!("[SchwirigkeitListe.from_name] name no encontrado: {}", name))
    }
}

// 0: einfag
// 1: normal
// 2: schwirig
#[derive(Debug, Clone)]
pub struct NewSchwirigkeitSchema {
    pub id: i32,
    pub schwirigkeit: String,
}

impl NewSchwirigkeitSchema {
    pub fn new(id: impl Into<i32>, schwirigkeit: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schwirigkeit: schwirigkeit.into(),
        }
    }
}
