use std::{
    collections::HashMap,
    result,
    sync::{LazyLock, Mutex},
};

use color_eyre::eyre::{Result, bail, eyre};

use crate::{
    db::{
        schemas::gram_type::{GramTypeSchema, NewGramTypeSchema, RawGramTypeSchema},
        traits::FromRaw,
    },
    helpers::time::string_2_datetime,
};

static HASH_VALUES: LazyLock<Mutex<HashMap<i32, GramTypeSchema>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

impl GramTypeSchema {
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

impl FromRaw<RawGramTypeSchema> for GramTypeSchema {
    fn from_raw(r: RawGramTypeSchema) -> Result<Self> {
        let created_at = string_2_datetime(Some(r.created_at)).unwrap();
        let deleted_at = string_2_datetime(r.deleted_at);

        Ok(GramTypeSchema {
            id: r.id,
            code: r.code,
            name: r.name,
            created_at,
            deleted_at,
        })
    }

    fn from_vec_raw(data: Vec<RawGramTypeSchema>) -> Result<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
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
