use color_eyre::eyre::{Result, bail};
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use crate::db::schemas::schwirigkeit_liste::{
    NewSchwirigkeitListeSchema as New, SchwirigkeitListeSchema as Schema,
};

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
            None => bail!("Schwirigkeit Liste no encontrado con el id: {}", id),
        }
    }

    pub fn from_name(name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        let hash = HASH_VALUES.lock().unwrap();
        let result = hash
            .iter()
            .find(|(_, value)| value.schwirigkeit == name)
            .map(|(_, value)| Self { ..value.clone() });

        match result {
            Some(v) => Ok(v),
            None => bail!("Schwirigkeit Liste no encontrado con el nombre: {}", name),
        }
    }
}

impl New {
    #[inline]
    pub fn new<I, S>(id: I, schwirigkeit: S) -> Self
    where
        I: Into<i32>,
        S: Into<String>,
    {
        Self {
            id: id.into(),
            schwirigkeit: schwirigkeit.into(),
        }
    }
}
