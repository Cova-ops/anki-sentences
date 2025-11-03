use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct GeschichtlichSetze {
    pub id: i32,
    pub setze_id: i32,
    pub result: bool, // 0: schlecht, 1: gut
    pub created_at: DateTime<Utc>,
}
