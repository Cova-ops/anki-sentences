use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use color_eyre::eyre::{Result, eyre};

use crate::{
    db::{
        schemas::gender_worte::{GenderWorteSchema, NewGenderWorteSchema, RawGenderWorteSchema},
        traits::FromRaw,
    },
    helpers::time::string_2_datetime,
};

static HASH_VALUES: LazyLock<Mutex<HashMap<i32, GenderWorteSchema>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

impl GenderWorteSchema {
    pub fn init_data(data: &[GenderWorteSchema]) -> Result<()> {
        let mut hash = HASH_VALUES.lock().unwrap();
        for d in data {
            hash.insert(d.id, d.clone());
        }
        Ok(())
    }

    pub fn from_id(id: impl Into<i32>) -> Result<Self> {
        let id = id.into();
        let hash = HASH_VALUES.lock().unwrap();
        hash.get(&id)
            .cloned()
            .ok_or_else(|| eyre!("[GenderWorteSchema.from_id] id no encontrado: {}", id))
    }
}

impl FromRaw<RawGenderWorteSchema> for GenderWorteSchema {
    fn from_raw(r: RawGenderWorteSchema) -> Result<Self> {
        let created_at = string_2_datetime(Some(r.created_at)).unwrap();
        let deleted_at = string_2_datetime(r.deleted_at);

        Ok(GenderWorteSchema {
            id: r.id,
            gender: r.gender,
            artikel: r.artikel,
            created_at,
            deleted_at,
        })
    }

    fn from_vec_raw(data: Vec<RawGenderWorteSchema>) -> Result<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
}

impl NewGenderWorteSchema {
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
