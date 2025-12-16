use color_eyre::eyre::{Result, bail};
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use crate::db::schemas::niveau_liste::{NewNiveauListeSchema as New, NiveauListeSchema as Schema};

static HASH_VALUES: LazyLock<Mutex<HashMap<i32, Schema>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

impl Schema {
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
        let result = hash.get(&id).cloned();
        match result {
            Some(v) => Ok(v),
            None => bail!("No se encontro Niveau Worte con id: {}", id),
        }
    }

    pub fn from_niveau(niveau: impl Into<String>) -> Result<Self> {
        let niveau = niveau.into();
        let hash = HASH_VALUES.lock().unwrap();
        let result = hash
            .iter()
            .find(|(_, val)| val.niveau == niveau)
            .map(|(_, val)| Self { ..val.clone() });

        match result {
            Some(v) => Ok(v),
            None => bail!("No se encontro Niveau Worte con el nombre: {}", niveau),
        }
    }
}

impl New {
    #[inline]
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
