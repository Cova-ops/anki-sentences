use color_eyre::eyre::{Result, bail};
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use crate::db::schemas::gram_type::{GramTypeSchema as Schema, NewGramTypeSchema as New};

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

    pub fn from_id<I>(id: I) -> Result<Self>
    where
        I: Into<i32>,
    {
        let id = id.into();
        let hash = HASH_VALUES.lock().unwrap();
        let result = hash.get(&id).cloned();
        match result {
            Some(v) => Ok(v),
            None => bail!("Gram Type no encontrado con el id: {}", id),
        }
    }

    pub fn from_code<S>(code: S) -> Result<Self>
    where
        S: Into<String>,
    {
        let code = code.into();
        let hash = HASH_VALUES.lock().unwrap();
        let result = hash
            .iter()
            .find(|(_, val)| val.code == code)
            .map(|(_, val)| Self { ..val.clone() });

        match result {
            Some(v) => Ok(v),
            None => bail!("Gram Type no encontrado con el código: {}", code),
        }
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
