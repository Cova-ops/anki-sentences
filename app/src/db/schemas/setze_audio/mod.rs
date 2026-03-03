mod input;
mod model;
mod schema;
mod snapshot;

pub use input::*;
pub use model::*;
pub use schema::*;

#[allow(unused_imports)]
pub use snapshot::*;

pub const CREATE_STR_TABLE_SETZE_AUDIO: &str = "
CREATE TABLE IF NOT EXISTS setze_audio(
    satz_id             INTEGER PRIMARY KEY,
    audio_name_es       TEXT,
    audio_name_de       TEXT,

    created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at          TEXT,

    FOREIGN KEY (satz_id) REFERENCES setze(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)";

pub const CREATE_STR_INDEX_SETZE_AUDIO: &str = "";
