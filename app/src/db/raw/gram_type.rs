use color_eyre::eyre::{self, Result};
use std::{collections::HashMap, sync::OnceLock};

use crate::db::schemas::gram_type::{GramTypeSchema as Schema, NewGramTypeSchema as New};

static HASH_VALUES: OnceLock<HashMap<i32, Schema>> = OnceLock::new();

impl Schema {
    pub fn init_data(data: &[Schema]) {
        let map: HashMap<i32, Schema> = data.iter().cloned().map(|s| (s.id, s)).collect();
        let _ = HASH_VALUES.set(map); // si ya estaba seteado, ignora o maneja error
    }

    pub fn from_id(id: i32) -> Result<Self> {
        let map = HASH_VALUES
            .get()
            .ok_or_else(|| eyre::eyre!("HASH_VALUES not initialized"))?;

        map.get(&id)
            .cloned()
            .ok_or_else(|| eyre::eyre!("Gram Type not founded with id: {}", id))
    }

    pub fn from_code(code: &str) -> Result<Self> {
        let map = HASH_VALUES
            .get()
            .ok_or_else(|| eyre::eyre!("HASH_VALUES not initialized"))?;

        map.iter()
            .find(|(_, val)| val.code == code)
            .map(|(_, val)| Self { ..val.clone() })
            .ok_or_else(|| eyre::eyre!("Gram Type not founded with code: {}", code))
    }
}

impl New {
    #[inline]
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
