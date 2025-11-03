use chrono::{DateTime, Utc};

use crate::db::SchwirigkeitListeSchema;

#[derive(Debug, Clone)]
pub struct Setze {
    pub id: i32,
    pub setze_spanisch: String,
    pub setze_deutsch: String,
    pub schwirig_id: SchwirigkeitListeSchema,
    pub thema: String,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct NewSetze {
    pub setze_spanisch: String,
    pub setze_deutsch: String,
    pub schwirig_id: i32,
    pub thema: String,
}

impl NewSetze {
    pub fn new(
        setze_spanisch: impl Into<String>,
        setze_deutsch: impl Into<String>,
        thema: impl Into<String>,
        schwirig_id: impl Into<i32>,
    ) -> Self {
        Self {
            setze_spanisch: setze_spanisch.into(),
            setze_deutsch: setze_deutsch.into(),
            schwirig_id: schwirig_id.into(),
            thema: thema.into(),
        }
    }
}
