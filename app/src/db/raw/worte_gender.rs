use color_eyre::eyre::{Result, bail};
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use crate::db::schemas::worte_gender::{NewWorteGenderSchema as New, WorteGenderSchema as Schema};

static HASH_VALUES: LazyLock<Mutex<HashMap<i32, Schema>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

impl Schema {
    pub fn init_data(data: &[Schema]) -> Result<()> {
        let mut hash = HASH_VALUES.lock().unwrap();
        for d in data {
            hash.insert(d.id, d.clone());
        }
        Ok(())
    }

    pub fn from_id(id: impl Into<i32>) -> Result<Self> {
        let id = id.into();
        let hash = HASH_VALUES.lock().unwrap();
        let result = hash.get(&id).cloned();
        match result {
            Some(v) => Ok(v),
            None => bail!("No se encontro GenderWorteSchema del id: {}", id),
        }
    }

    pub fn from_gender(gender: impl Into<String>) -> Result<Self> {
        let gender = gender.into();
        let hash = HASH_VALUES.lock().unwrap();
        let result = hash
            .iter()
            .find(|(_, v)| v.gender == gender)
            .map(|(_, v)| Self { ..v.clone() });

        match result {
            Some(v) => Ok(v),
            None => bail!("No se encontro GenderWorteSchema del genero: {}", gender),
        }
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
