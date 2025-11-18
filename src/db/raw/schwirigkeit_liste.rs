use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
};

use color_eyre::eyre::{Result, eyre};

use crate::{
    db::{
        schemas::schwirigkeit_liste::{
            NewSchwirigkeitListeSchema, RawSchwirigkeitListeSchema, SchwirigkeitListeSchema,
        },
        traits::FromRaw,
    },
    helpers::time::string_2_datetime,
};

static HASH_VALUES: LazyLock<Mutex<HashMap<i32, SchwirigkeitListeSchema>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

impl SchwirigkeitListeSchema {
    pub fn init_data(data: &[SchwirigkeitListeSchema]) -> Result<()> {
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
            .ok_or_else(|| eyre!("[SchwirigkeitListe.from_id] id no encontrado: {}", id))
    }

    pub fn from_name(name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        let hash = HASH_VALUES.lock().unwrap();
        hash.iter()
            .find(|(_, value)| value.schwirigkeit == name)
            .map(|(_, value)| Self { ..value.clone() })
            .ok_or_else(|| eyre!("[SchwirigkeitListe.from_name] name no encontrado: {}", name))
    }
}

impl FromRaw<RawSchwirigkeitListeSchema> for SchwirigkeitListeSchema {
    fn from_raw(r: RawSchwirigkeitListeSchema) -> Result<Self> {
        let created_at = string_2_datetime(Some(r.created_at)).unwrap();
        let deleted_at = string_2_datetime(r.deleted_at);

        Ok(SchwirigkeitListeSchema {
            id: r.id,
            schwirigkeit: r.schwirigkeit,
            created_at,
            deleted_at,
        })
    }

    fn from_vec_raw(data: Vec<RawSchwirigkeitListeSchema>) -> Result<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
}

impl NewSchwirigkeitListeSchema {
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
