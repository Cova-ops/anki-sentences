use std::str::FromStr;

use chrono::{DateTime, Utc};

use crate::{
    db::schemas::wort_review::{EnumReviewDirection, SchemaWortReview},
    helpers::{error_handler::InvalidValueError, time::string_2_datetime},
};

#[derive(Debug, Clone)]
pub struct ModelWortReview {
    pub wort_id: i32,

    pub direction: EnumReviewDirection,
    pub interval: u32,
    pub ease_factor: f64,
    pub repetitions: u32,
    pub last_review: DateTime<Utc>,
    pub next_review: DateTime<Utc>,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TryFrom<SchemaWortReview> for ModelWortReview {
    type Error = Vec<InvalidValueError>;

    fn try_from(value: SchemaWortReview) -> Result<Self, Self::Error> {
        let mut errs = vec![];

        let wort_id = value.wort_id;

        let direction = match EnumReviewDirection::from_str(&value.direction) {
            Ok(v) => Some(v),
            Err(e) => {
                errs.push(e);
                None
            }
        };

        let interval = value.interval;
        let ease_factor = value.ease_factor;
        let repetitions = value.repetitions;

        let last_review = match string_2_datetime(value.last_review) {
            Ok(v) => Some(v),
            Err(e) => {
                errs.push(e);
                None
            }
        };
        let next_review = match string_2_datetime(value.next_review) {
            Ok(v) => Some(v),
            Err(e) => {
                errs.push(e);
                None
            }
        };

        let created_at = match string_2_datetime(value.created_at) {
            Ok(v) => Some(v),
            Err(e) => {
                errs.push(e);
                None
            }
        };

        let deleted_at = if let Some(date) = value.deleted_at {
            match string_2_datetime(date) {
                Ok(v) => Some(v.clone()),
                Err(e) => {
                    errs.push(e);
                    None
                }
            }
        } else {
            None
        };

        if !errs.is_empty() {
            return Err(errs);
        }

        Ok(Self {
            wort_id,
            direction: direction.unwrap(),
            interval,
            ease_factor,
            repetitions,
            last_review: last_review.unwrap(),
            next_review: next_review.unwrap(),
            created_at: created_at.unwrap(),
            deleted_at,
        })
    }
}

impl ModelWortReview {
    pub fn try_from_iter(
        value: impl IntoIterator<Item = SchemaWortReview>,
    ) -> Result<Vec<ModelWortReview>, Vec<InvalidValueError>> {
        let mut errs = vec![];
        let mut oks = vec![];

        for v in value {
            match ModelWortReview::try_from(v) {
                Ok(v) => oks.push(v),
                Err(mut e) => errs.append(&mut e),
            }
        }

        if errs.is_empty() { Ok(oks) } else { Err(errs) }
    }
}

#[cfg(test)]
mod tests_model_wort_review {
    use super::*;

    fn ok_schema() -> SchemaWortReview {
        SchemaWortReview {
            wort_id: 10,
            direction: "es_to_de".to_string(),
            interval: 3,
            ease_factor: 2.5,
            repetitions: 4,
            last_review: "2025-12-04 17:44:37".to_string(),
            next_review: "2025-12-10 10:00:00".to_string(),
            created_at: "2025-12-04 17:44:37".to_string(),
            deleted_at: None,
        }
    }

    #[test]
    fn try_from_ok() {
        let s = ok_schema();
        let m = ModelWortReview::try_from(s).expect("should convert");

        assert_eq!(m.wort_id, 10);
        assert_eq!(m.direction, EnumReviewDirection::ES2DE);
        assert_eq!(m.interval, 3);
        assert!((m.ease_factor - 2.5).abs() < f64::EPSILON);
        assert_eq!(m.repetitions, 4);

        // sanity: parsed datetimes exist
        assert!(m.last_review <= m.next_review);
        assert!(m.created_at <= m.next_review);
        assert!(m.deleted_at.is_none());
    }

    #[test]
    fn try_from_ok_with_deleted_at() {
        let mut s = ok_schema();
        s.deleted_at = Some("2025-12-31 23:59:59".to_string());

        let m = ModelWortReview::try_from(s).expect("should convert");
        assert!(m.deleted_at.is_some());
    }

    #[test]
    fn try_from_err_invalid_direction() {
        let mut s = ok_schema();
        s.direction = "bad_direction".to_string();

        let err = ModelWortReview::try_from(s).expect_err("should fail");
        assert!(err.iter().any(|e| e.field == "EnumReviewDirection"));
        let e = err
            .iter()
            .find(|e| e.field == "EnumReviewDirection")
            .unwrap();

        assert!(e.message.contains("bad_direction"));
        assert_eq!(
            e.valid_options.as_deref(),
            Some(&["es_to_de", "de_to_es"][..])
        );
    }

    #[test]
    fn try_from_err_invalid_dates_accumulates() {
        let mut s = ok_schema();
        s.last_review = "not-a-date".to_string();
        s.next_review = "also-bad".to_string();

        let err = ModelWortReview::try_from(s).expect_err("should fail");
        // should include at least the datetime errors (could be 2+ depending on your parser)
        assert!(err.iter().any(|e| e.field == "datetime"));
        assert!(err.len() >= 2);
    }

    #[test]
    fn try_from_iter_all_ok() {
        let v = vec![ok_schema(), ok_schema()];
        let out = ModelWortReview::try_from_iter(v).expect("all ok");
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn try_from_iter_collects_errors() {
        let mut bad = ok_schema();
        bad.direction = "bad_direction".to_string();

        let v = vec![ok_schema(), bad];
        let err = ModelWortReview::try_from_iter(v).expect_err("should error");
        assert!(err.iter().any(|e| e.field == "EnumReviewDirection"));
    }
}
