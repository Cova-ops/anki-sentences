use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use color_eyre::eyre::{Result, eyre};

use crate::{
    db::{
        schemas::niveau_worte::{NewNiveauWorteSchema, NiveauWorteSchema, RawNiveauWorteSchema},
        traits::FromRaw,
    },
    helpers::time::string_2_datetime,
};

static HASH_VALUES: LazyLock<Mutex<HashMap<i32, NiveauWorteSchema>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

impl NiveauWorteSchema {
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
        hash.get(&id)
            .cloned()
            .ok_or_else(|| eyre!("[NiveauWorteSchema.from_id] id no encontrado: {}", id))
    }
}

impl FromRaw<RawNiveauWorteSchema> for NiveauWorteSchema {
    fn from_raw(r: RawNiveauWorteSchema) -> Result<Self> {
        let created_at = string_2_datetime(Some(r.created_at)).unwrap();
        let deleted_at = string_2_datetime(r.deleted_at);

        Ok(NiveauWorteSchema {
            id: r.id,
            niveau: r.niveau,
            created_at,
            deleted_at,
        })
    }

    fn from_vec_raw(data: Vec<RawNiveauWorteSchema>) -> Result<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
}

impl NewNiveauWorteSchema {
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
