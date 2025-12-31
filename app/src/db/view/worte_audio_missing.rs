use color_eyre::eyre::Result;
use sql_model::FromRaw;

#[derive(Debug, Clone)]
pub struct WorteAudioMissingSchema {
    pub id: i32,
    pub wort_es: String,
    pub wort_de: String,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,
}

impl FromRaw<RawWorteAudioMissingSchema> for WorteAudioMissingSchema {
    fn from_raw(r: RawWorteAudioMissingSchema) -> Result<Self> {
        Ok(WorteAudioMissingSchema {
            id: r.id,
            wort_es: r.wort_es,
            wort_de: r.wort_de,
            audio_name_es: r.audio_name_es,
            audio_name_de: r.audio_name_de,
        })
    }

    fn from_vec_raw(data: Vec<RawWorteAudioMissingSchema>) -> Result<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
}

pub struct RawWorteAudioMissingSchema {
    pub id: i32,
    pub wort_es: String,
    pub wort_de: String,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,
}

impl RawWorteAudioMissingSchema {
    pub fn from_sql(r: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: r.get(0)?,
            wort_es: r.get(1)?,
            wort_de: r.get(2)?,
            audio_name_es: r.get(3)?,
            audio_name_de: r.get(4)?,
        })
    }
}
