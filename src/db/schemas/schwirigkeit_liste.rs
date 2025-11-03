use chrono::{DateTime, Utc};
use color_eyre::eyre::{Result, eyre};

use crate::db::SchwirigkeitListeFetchAll;

// 0: einfag
// 1: normal
// 2: schwirig
#[derive(Debug, Clone)]
pub struct SchwirigkeitListe {
    pub id: i32,
    pub schwirigkeit: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl SchwirigkeitListe {
    pub fn from_id(id: impl Into<i32>) -> Result<Self> {
        let id = id.into();
        SchwirigkeitListeFetchAll()
            .get(&id)
            .cloned()
            .ok_or_else(|| eyre!("[SchwirigkeitListe.from_id] id no encontrado: {}", id))
    }

    pub fn from_name(name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        SchwirigkeitListeFetchAll()
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
pub struct NewSchwirigkeit {
    pub id: i32,
    pub schwirigkeit: String,
}

impl NewSchwirigkeit {
    pub fn new(id: impl Into<i32>, schwirigkeit: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            schwirigkeit: schwirigkeit.into(),
        }
    }
}
