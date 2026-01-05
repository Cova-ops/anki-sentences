use color_eyre::eyre::{self, Result};
use std::{
    collections::HashMap,
    sync::OnceLock,
};

use crate::db::schemas::worte_gender::{NewWorteGenderSchema as New, WorteGenderSchema as Schema};

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
            .ok_or_else(|| eyre::eyre!("Worte Gender not founded with id: {}", id))
    }

    pub fn from_gender(gender: &str) -> Result<Self> {
        let map = HASH_VALUES
            .get()
            .ok_or_else(|| eyre::eyre!("HASH_VALUES not initialized"))?;

        map.iter()
            .find(|(_, val)| val.gender == gender)
            .map(|(_, val)| Self { ..val.clone() })
            .ok_or_else(|| eyre::eyre!("Gender Worte not founded with gender: {}", gender))
    }
}

impl New {
    #[inline]
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
