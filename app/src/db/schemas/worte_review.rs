use chrono::{DateTime, Utc};
use color_eyre::eyre;
use sql_model::SqlModel;

use crate::services::tts::eleven_labs::LanguageVoice;

pub const CREATE_STR_TABLE_WORTE_REVIEW: &str = "
CREATE TABLE IF NOT EXISTS worte_review (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    wort_id         INTEGER NOT NULL,
    direction       TEXT NOT NULL,
    interval        INTEGER NOT NULL,
    ease_factor     REAL    NOT NULL,
    repetitions     INTEGER NOT NULL,
    last_review     TEXT NOT NULL,
    next_review     TEXT NOT NULL,
    created_at      TEXT DEFAULT CURRENT_TIMESTAMP,
    deleted_at      TEXT,
    FOREIGN KEY(wort_id) REFERENCES worte(id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
";

pub const CREATE_STR_INDEX_WORTE_REVIEW: &str = "
    CREATE INDEX IF NOT EXISTS idx_worte_review_wort_id ON worte_review(wort_id);
    CREATE INDEX IF NOT EXISTS idx_worte_review_next_review ON worte_review(next_review);
    CREATE UNIQUE INDEX IF NOT EXISTS idx_worte_review_unique ON worte_review(wort_id, direction);
";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewDirection {
    ES2DE,
    DE2ES,
}

impl ReviewDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReviewDirection::ES2DE => "es_to_de",
            ReviewDirection::DE2ES => "de_to_es",
        }
    }

    pub fn to_string(&self) -> String {
        Self::as_str(&self).to_owned()
    }

    pub fn from_lang(lang: LanguageVoice) -> Self {
        match lang {
            LanguageVoice::Spanisch => Self::ES2DE,
            LanguageVoice::Deutsch => Self::DE2ES,
        }
    }
}

impl TryFrom<&str> for ReviewDirection {
    type Error = eyre::Report;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "es_to_de" => Ok(ReviewDirection::ES2DE),
            "de_to_es" => Ok(ReviewDirection::DE2ES),

            _ => eyre::bail!("Invalid review direction string: {}", value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorteReviewSchema {
    pub id: i32,

    pub wort_id: i32,
    pub direction: ReviewDirection,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: DateTime<Utc>,
    pub next_review: DateTime<Utc>,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, SqlModel)]
#[sql(insert(
    wort_id,
    direction,
    interval,
    ease_factor,
    repetitions,
    last_review,
    next_review
))]
pub struct NewWorteReviewSchema {
    pub wort_id: i32,
    pub direction: String,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: String, // DateTime<Utc>
    pub next_review: String, // DateTime<Utc>
}

#[derive(Debug, SqlModel)]
#[sql(raw(
    id,
    wort_id,
    direction,
    interval,
    ease_factor,
    repetitions,
    last_review,
    next_review,
    created_at,
    deleted_at
))]
pub struct RawWorteReviewSchema {
    pub id: i32,
    pub wort_id: i32,
    pub direction: String,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: String,
    pub next_review: String,

    // Generic
    pub created_at: String,
    pub deleted_at: Option<String>,
}
