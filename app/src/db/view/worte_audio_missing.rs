use crate::{
    db::traits::from_raw::{FromRaw, FromSql},
    helpers::error_handler::{DbError, ValidationError},
};

#[derive(Debug, Clone)]
pub struct WorteAudioMissingSchema {
    pub id: i32,
    pub wort_es: String,
    pub wort_de: String,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,
}

impl FromRaw<RawWorteAudioMissingSchema> for WorteAudioMissingSchema {
    type Error = ValidationError;

    fn from_raw<'a>(r: &'a RawWorteAudioMissingSchema) -> Result<Self, Self::Error> {
        Ok(WorteAudioMissingSchema {
            id: r.id,
            wort_es: r.wort_es.clone(),
            wort_de: r.wort_de.clone(),
            audio_name_es: r.audio_name_es.clone(),
            audio_name_de: r.audio_name_de.clone(),
        })
    }

    fn from_vec_raw<'a, I>(data: I) -> Result<Vec<Self>, Self::Error>
    where
        I: IntoIterator<Item = &'a RawWorteAudioMissingSchema>,
    {
        data.iter().map(Self::from_raw).collect()
    }
}

pub struct RawWorteAudioMissingSchema {
    pub id: i32,
    pub wort_es: String,
    pub wort_de: String,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,
}

impl FromSql for RawWorteAudioMissingSchema {
    fn from_sql(r: &rusqlite::Row<'_>) -> Result<Self, DbError> {
        Ok(Self {
            id: r.get(0)?,
            wort_es: r.get(1)?,
            wort_de: r.get(2)?,
            audio_name_es: r.get(3)?,
            audio_name_de: r.get(4)?,
        })
    }
}
