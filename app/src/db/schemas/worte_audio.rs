use chrono::{DateTime, Utc};
use sql_model::SqlModel;

pub const CREATE_STR_TABLE_WORTE_AUDIO: &str = "
CREATE TABLE IF NOT EXISTS worte_audio(
    wort_id      INTEGER PRIMARY KEY,
    file_path    TEXT NOT NULL,
    voice_id     TEXT NOT NULL,

    -- Generic
    created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at          TEXT,

    FOREIGN KEY (wort_id) REFERENCES worte(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)";

pub const CREATE_STR_INDEX_WORTE_AUDIO: &str = "
    CREATE INDEX IF NOT EXISTS idx_worte_audio_voice_id ON worte_audio(voice_id);
";

#[derive(Debug, Clone, SqlModel)]
#[sql(raw_type = "RawWorteAudioSchema")]
#[sql(raw(wort_id, file_path, voice_id, created_at, deleted_at))]
pub struct WorteAudioSchema {
    pub wort_id: i32,
    pub file_path: String,
    pub voice_id: String,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, SqlModel)]
#[sql(insert(wort_id, file_path, voice_id))]
pub struct NewWorteAudioSchema {
    pub wort_id: i32,
    pub file_path: String,
    pub voice_id: String,
}

#[derive(Debug, SqlModel)]
#[sql(raw(wort_id, file_path, voice_id, created_at, deleted_at))]
pub struct RawWorteAudioSchema {
    pub wort_id: i32,
    pub file_path: String,
    pub voice_id: String,

    // Generic
    pub created_at: String,
    pub deleted_at: Option<String>,
}
