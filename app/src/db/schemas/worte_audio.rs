use chrono::{DateTime, Utc};
use sql_model::SqlModel;

pub const CREATE_STR_TABLE_WORTE_AUDIO: &str = "
CREATE TABLE IF NOT EXISTS worte_audio(
    wort_id         INTEGER PRIMARY KEY,
    audio_name_es   TEXT NOT NULL,
    audio_name_de   TEXT NOT NULL,

    -- Generic
    created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at          TEXT,

    FOREIGN KEY (wort_id) REFERENCES worte(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)";

pub const CREATE_STR_INDEX_WORTE_AUDIO: &str = "
";

#[derive(Debug, Clone, SqlModel)]
#[sql(raw_type = "RawWorteAudioSchema")]
#[sql(raw(wort_id, audio_name_es, audio_name_de, created_at, deleted_at))]
pub struct WorteAudioSchema {
    pub wort_id: i32,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, SqlModel)]
#[sql(insert(wort_id, audio_name_es, audio_name_de))]
pub struct NewWorteAudioSchema {
    pub wort_id: i32,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,
}

#[derive(Debug, SqlModel)]
#[sql(raw(wort_id, audio_name_es, audio_name_de, created_at, deleted_at))]
pub struct RawWorteAudioSchema {
    pub wort_id: i32,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,

    // Generic
    pub created_at: String,
    pub deleted_at: Option<String>,
}
