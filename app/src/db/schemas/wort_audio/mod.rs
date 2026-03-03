mod input;
mod model;
mod schema;
mod snapshot;

pub use input::*;
pub use model::*;
pub use schema::*;

#[allow(unused_imports)]
pub use snapshot::*;

pub const CREATE_STR_TABLE_WORTE_AUDIO: &str = "
CREATE TABLE IF NOT EXISTS worte_audio(
    wort_id         INTEGER PRIMARY KEY,
    audio_name_es   TEXT,
    audio_name_de   TEXT,

    -- Generic
    created_at          TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at          TEXT,

    FOREIGN KEY (wort_id) REFERENCES worte(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)";

pub const CREATE_STR_INDEX_WORTE_AUDIO: &str = "
";
