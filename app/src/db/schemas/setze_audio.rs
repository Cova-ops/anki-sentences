use chrono::{DateTime, Utc};
use sql_model::SqlModel;

pub const CREATE_STR_TABLE_SETZE_AUDIO: &str = "
CREATE TABLE IF NOT EXISTS setze_audio(
    satz_id      INTEGER PRIMARY KEY,
    file_path    TEXT NOT NULL,
    voice_id     TEXT NOT NULL,

    -- Generic
    created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at          TEXT,

    FOREIGN KEY (satz_id) REFERENCES setze(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)";

pub const CREATE_STR_INDEX_SETZE_AUDIO: &str = "
    CREATE INDEX IF NOT EXISTS idx_setze_audio_voice_id ON setze_audio(voice_id);
";

#[derive(Debug, Clone, SqlModel)]
#[sql(raw_type = "RawSetzeAudioSchema")]
#[sql(raw(satz_id, file_path, voice_id, created_at, deleted_at))]
pub struct SetzeAudioSchema {
    pub satz_id: i32,
    pub file_path: String,
    pub voice_id: String,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, SqlModel)]
#[sql(insert(satz_id, file_path, voice_id))]
pub struct NewSetzeAudioSchema {
    pub satz_id: i32,
    pub file_path: String,
    pub voice_id: String,
}

#[derive(Debug, SqlModel)]
#[sql(raw(satz_id, file_path, voice_id, created_at, deleted_at))]
pub struct RawSetzeAudioSchema {
    pub satz_id: i32,
    pub file_path: String,
    pub voice_id: String,

    // Generic
    pub created_at: String,
    pub deleted_at: Option<String>,
}
