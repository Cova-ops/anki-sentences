use chrono::{DateTime, Utc};
use color_eyre::eyre;
use sql_model::SqlModel;

pub const CREATE_STR_TABLE_WORTE_GENDER: &str = "
    CREATE TABLE IF NOT EXISTS worte_gender (
        id                  INTEGER PRIMARY KEY AUTOINCREMENT,
        gender              TEXT NOT NULL,                         
        artikel             TEXT NOT NULL,

        -- Generic
        created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
        deleted_at          TEXT
    )
";

pub const CREATE_STR_INDEX_WORTE_GENDER: &str = "
    CREATE INDEX IF NOT EXISTS idx_worte_gender_created_at ON worte_gender(created_at);
";

pub enum GenderGermanListe {
    Der,
    Das,
    Die,
}

impl GenderGermanListe {
    pub fn to_string(&self) -> String {
        match self {
            GenderGermanListe::Der => "der".to_owned(),
            GenderGermanListe::Das => "das".to_owned(),
            GenderGermanListe::Die => "die".to_owned(),
        }
    }
}

impl TryFrom<&str> for GenderGermanListe {
    type Error = eyre::Report;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "der" => Ok(GenderGermanListe::Der),
            "das" => Ok(GenderGermanListe::Das),
            "die" => Ok(GenderGermanListe::Die),

            _ => eyre::bail!("Gender not founded for: {}", s),
        }
    }
}

// 0 - Maskuline - der
// 1 - Femenin - die
// 2 - Neutrum - das
// 3 - Plural - die
#[derive(Debug, Clone, SqlModel, PartialEq, Eq)]
#[sql(raw_type = "RawWorteGenderSchema")]
#[sql(raw(id, gender, artikel, created_at, deleted_at))]
pub struct WorteGenderSchema {
    pub id: i32,
    pub gender: String,
    pub artikel: String,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, SqlModel)]
#[sql(insert(id, gender, artikel))]
pub struct NewWorteGenderSchema {
    pub id: i32,
    pub gender: String,
    pub artikel: String,
}

#[derive(Debug, SqlModel)]
#[sql(raw(id, gender, artikel, created_at, deleted_at))]
pub struct RawWorteGenderSchema {
    pub id: i32,
    pub gender: String,
    pub artikel: String,
    pub created_at: String,
    pub deleted_at: Option<String>,
}
