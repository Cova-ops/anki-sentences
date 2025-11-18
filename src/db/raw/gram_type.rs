use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use color_eyre::eyre::{Result, eyre};

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
        hash.get(&id)
            .cloned()
            .ok_or_else(|| eyre!("[GramTypeSchema.from_id] id no encontrado: {}", id))
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
