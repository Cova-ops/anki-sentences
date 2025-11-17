use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct WortGramTypeSchema {
    pub id_wort: i32,
    pub id_gram_type: i32,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct NewWortGramTypeSchema {
    pub id_wort: i32,
    pub id_gram_type: i32,
}
