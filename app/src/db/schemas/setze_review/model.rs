use chrono::{DateTime, Utc};

use crate::{
    db::schemas::setze_review::SchemaSetzeReview,
    helpers::{error_handler::InvalidValueError, time::string_2_datetime},
};

#[derive(Debug, Clone)]
pub struct ModelSetzeReview {
    pub id: i32,

    pub satz_id: i32,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: DateTime<Utc>,
    pub next_review: DateTime<Utc>,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TryFrom<SchemaSetzeReview> for ModelSetzeReview {
    type Error = Vec<InvalidValueError>;

    fn try_from(value: SchemaSetzeReview) -> Result<Self, Self::Error> {
        let mut errs: Vec<_> = vec![];

        let id = value.id;
        let satz_id = value.satz_id;
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
                Err(e) => errs.push(e),
            }
        } else {
            None
        };

        if !errs.is_empty() {
            return Err(errs);
        }

        Ok(Self {
            id,
            satz_id,
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

impl ModelSetzeReview {
    fn try_from_iter(
        value: impl IntoIterator<Item = SchemaSetzeReview>,
    ) -> Result<Vec<ModelSetzeReview>, Vec<InvalidValueError>> {
        let mut errs = vec![];
        let mut oks = vec![];

        for v in value {
            match ModelSetzeReview::try_from(v) {
                Ok(v) => oks.push(v),
                Err(mut e) => errs.append(&mut e),
            }
        }

        if errs.is_empty() { Ok(oks) } else { Err(errs) }
    }
}

#[cfg(test)]
mod tests_model_setze_review {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn schema_ok() -> SchemaSetzeReview {
        SchemaSetzeReview {
            id: 1,
            satz_id: 10,
            interval: 3,
            ease_factor: 2.5,
            repetitions: 7,
            last_review: "2025-12-04 17:44:37".to_string(),
            next_review: "2025-12-10 08:10:00".to_string(),
            created_at: "2025-12-01 00:00:00".to_string(),
            deleted_at: None,
        }
    }

    #[test]
    fn try_from_ok() {
        let raw = schema_ok();
        let model = ModelSetzeReview::try_from(raw).expect("should convert");

        assert_eq!(model.id, 1);
        assert_eq!(model.satz_id, 10);
        assert_eq!(model.interval, 3);
        assert!((model.ease_factor - 2.5).abs() < f32::EPSILON);
        assert_eq!(model.repetitions, 7);

        assert_eq!(
            model.last_review,
            Utc.with_ymd_and_hms(2025, 12, 4, 17, 44, 37).unwrap()
        );
        assert_eq!(
            model.next_review,
            Utc.with_ymd_and_hms(2025, 12, 10, 8, 10, 0).unwrap()
        );
        assert_eq!(
            model.created_at,
            Utc.with_ymd_and_hms(2025, 12, 1, 0, 0, 0).unwrap()
        );
        assert_eq!(model.deleted_at, None);
    }

    #[test]
    fn try_from_ok_with_deleted_at() {
        let mut raw = schema_ok();
        raw.deleted_at = Some("2025-12-31 23:59:59".to_string());

        let model = ModelSetzeReview::try_from(raw).expect("should convert");
        assert_eq!(
            model.deleted_at,
            Some(Utc.with_ymd_and_hms(2025, 12, 31, 23, 59, 59).unwrap())
        );
    }

    #[test]
    fn try_from_collects_errors_for_invalid_dates() {
        let mut raw = schema_ok();
        raw.last_review = "BAD_DATE".to_string();
        raw.next_review = "ALSO_BAD".to_string();
        raw.created_at = "NOPE".to_string();
        raw.deleted_at = Some("WONT_PARSE".to_string());

        let err = ModelSetzeReview::try_from(raw).unwrap_err();
        // 4 fechas malas => 4 errores acumulados
        assert_eq!(err.len(), 4);

        // si quieres, verifica que son del tipo "datetime"
        assert!(
            err.iter()
                .all(|e: &InvalidValueError| e.field == "datetime")
        );
    }

    #[test]
    fn try_from_iter_ok() {
        let a = schema_ok();
        let mut b = schema_ok();
        b.id = 2;
        b.satz_id = 11;

        let res = ModelSetzeReview::try_from_iter([a, b]).expect("should convert");
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].id, 1);
        assert_eq!(res[1].id, 2);
    }

    #[test]
    fn try_from_iter_accumulates_errors() {
        let mut a = schema_ok();
        a.last_review = "BAD".to_string();

        let mut b = schema_ok();
        b.id = 2;
        b.next_review = "BAD".to_string();
        b.created_at = "BAD".to_string();

        let err = ModelSetzeReview::try_from_iter(vec![a, b]).unwrap_err();
        // a: 1 error, b: 2 errores => total 3
        assert_eq!(err.len(), 3);
        assert!(err.iter().all(|e| e.field == "datetime"));
    }
}
