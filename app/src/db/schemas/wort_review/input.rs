use chrono::{DateTime, Utc};

use crate::{db::schemas::wort_review::EnumReviewDirection, helpers::time::datetime_2_string};

#[derive(Debug)]
pub(in crate::db) struct SqlWortReview {
    pub wort_id: i32,
    pub direction: String,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: String,
    pub next_review: String,
}

#[derive(Debug)]
pub struct InputWortReview {
    pub wort_id: i32,
    pub direction: EnumReviewDirection,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: DateTime<Utc>,
    pub next_review: DateTime<Utc>,
}

impl From<InputWortReview> for SqlWortReview {
    fn from(value: InputWortReview) -> Self {
        Self {
            wort_id: value.wort_id,
            direction: value.direction.as_str().to_string(),
            interval: value.interval,
            ease_factor: value.ease_factor,
            repetitions: value.repetitions,
            last_review: datetime_2_string(value.last_review),
            next_review: datetime_2_string(value.next_review),
        }
    }
}

#[cfg(test)]
mod tests_sql_wort_review {
    use super::*;
    use chrono::TimeZone;

    fn dt1() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2025, 12, 4, 17, 44, 37).unwrap()
    }
    fn dt2() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 1, 10, 8, 0, 1).unwrap()
    }

    #[test]
    fn from_input_converts_all_fields() {
        let input = InputWortReview {
            wort_id: 123,
            direction: EnumReviewDirection::ES2DE,
            interval: 7,
            ease_factor: 2.45,
            repetitions: 3,
            last_review: dt1(),
            next_review: dt2(),
        };

        let sql: SqlWortReview = input.into();

        assert_eq!(sql.wort_id, 123);
        assert_eq!(
            sql.direction,
            EnumReviewDirection::ES2DE.as_str().to_string()
        );
        assert_eq!(sql.interval, 7);
        assert!((sql.ease_factor - 2.45).abs() < f32::EPSILON);
        assert_eq!(sql.repetitions, 3);

        assert_eq!(sql.last_review, datetime_2_string(dt1()));
        assert_eq!(sql.next_review, datetime_2_string(dt2()));
    }

    #[test]
    fn from_input_direction_is_the_expected_string() {
        let input = InputWortReview {
            wort_id: 1,
            direction: EnumReviewDirection::DE2ES,
            interval: 1,
            ease_factor: 1.3,
            repetitions: 0,
            last_review: dt1(),
            next_review: dt2(),
        };

        let sql: SqlWortReview = input.into();

        assert_eq!(sql.direction, EnumReviewDirection::DE2ES.as_str());
    }
}
