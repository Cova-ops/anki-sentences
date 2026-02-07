use chrono::{DateTime, Utc};

use crate::helpers::time::datetime_2_string;

#[derive(Debug)]
pub(in crate::db) struct SqlSetzeReview {
    pub satz_id: i32,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: String, // DateTime<Utc>
    pub next_review: String, // DateTime<Utc>
}

#[derive(Debug)]
pub struct InputSetzeReview {
   pub satz_id: i32,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: DateTime<Utc>,
    pub next_review: DateTime<Utc>,
}

impl From<InputSetzeReview> for SqlSetzeReview {
    fn from(value: InputSetzeReview) -> Self {
        Self {
            satz_id: value.satz_id,
            interval: value.interval,
            ease_factor: value.ease_factor,
            repetitions: value.repetitions,
            last_review: datetime_2_string(value.last_review),
            next_review: datetime_2_string(value.next_review),
        }
    }
}

#[cfg(test)]
mod tests_sql_setze_review {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn from_input_converts_all_fields_correctly() {
        let last_review = Utc.with_ymd_and_hms(2025, 1, 10, 12, 30, 0).unwrap();
        let next_review = Utc.with_ymd_and_hms(2025, 1, 15, 8, 0, 0).unwrap();

        let input = InputSetzeReview {
            satz_id: 42,
            interval: 5,
            ease_factor: 2.5,
            repetitions: 3,
            last_review,
            next_review,
        };

        let sql: SqlSetzeReview = input.into();

        assert_eq!(sql.satz_id, 42);
        assert_eq!(sql.interval, 5);
        assert_eq!(sql.ease_factor, 2.5);
        assert_eq!(sql.repetitions, 3);

        // Fechas convertidas correctamente a String
        assert_eq!(sql.last_review, datetime_2_string(last_review));
        assert_eq!(sql.next_review, datetime_2_string(next_review));
    }
}
