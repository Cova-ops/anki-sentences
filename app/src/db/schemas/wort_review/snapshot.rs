use chrono::{DateTime, Utc};

use crate::db::schemas::wort_review::{EnumReviewDirection, ModelWortReview, SchemaWortReview};

#[derive(Debug)]
pub struct SnapshotWortReview {
    pub wort_id: i32,
    pub direction: EnumReviewDirection,
    pub interval: u32,
    pub ease_factor: f64,
    pub repetitions: u32,
    pub last_review: DateTime<Utc>,
    pub next_review: DateTime<Utc>,

    // Generic
    pub created_at: &'static str,
    pub deleted_at: Option<&'static str>,
}

impl From<ModelWortReview> for SnapshotWortReview {
    fn from(value: ModelWortReview) -> Self {
        Self {
            wort_id: value.wort_id,
            direction: value.direction,
            interval: value.interval,
            ease_factor: value.ease_factor,
            repetitions: value.repetitions,
            last_review: value.last_review,
            next_review: value.next_review,

            // Generic
            created_at: "<created_at>",
            deleted_at: value.deleted_at.as_ref().map(|_| "<deleted_at>"),
        }
    }
}

impl From<SchemaWortReview> for SnapshotWortReview {
    /// Don't use this in prod
    /// It doesn't handle errors
    fn from(value: SchemaWortReview) -> Self {
        let model = ModelWortReview::try_from(value).unwrap();
        model.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_model {
        use super::*;

        fn model_ok(deleted: bool) -> ModelWortReview {
            ModelWortReview {
                wort_id: 10,
                direction: EnumReviewDirection::ES2DE,
                interval: 4,
                ease_factor: 2.5,
                repetitions: 3,
                last_review: DateTime::<Utc>::from_timestamp(1_700_000_000, 0).unwrap(),
                next_review: DateTime::<Utc>::from_timestamp(1_700_086_400, 0).unwrap(),
                created_at: DateTime::<Utc>::from_timestamp(1_699_000_000, 0).unwrap(),
                deleted_at: if deleted {
                    Some(DateTime::<Utc>::from_timestamp(1_700_100_000, 0).unwrap())
                } else {
                    None
                },
            }
        }

        #[test]
        fn snapshot_from_model_without_deleted_at() {
            let model = model_ok(false);

            let snap = SnapshotWortReview::from(model);

            assert_eq!(snap.wort_id, 10);
            assert_eq!(snap.direction, EnumReviewDirection::ES2DE);
            assert_eq!(snap.interval, 4);
            assert_eq!(snap.ease_factor, 2.5);
            assert_eq!(snap.repetitions, 3);

            assert_eq!(
                snap.last_review,
                DateTime::<Utc>::from_timestamp(1_700_000_000, 0).unwrap()
            );
            assert_eq!(
                snap.next_review,
                DateTime::<Utc>::from_timestamp(1_700_086_400, 0).unwrap()
            );

            assert_eq!(snap.created_at, "<created_at>");
            assert!(snap.deleted_at.is_none());
        }

        #[test]
        fn snapshot_from_model_with_deleted_at() {
            let model = model_ok(true);

            let snap = SnapshotWortReview::from(model);

            assert_eq!(snap.wort_id, 10);
            assert_eq!(snap.direction, EnumReviewDirection::ES2DE);
            assert_eq!(snap.interval, 4);
            assert_eq!(snap.ease_factor, 2.5);
            assert_eq!(snap.repetitions, 3);

            assert_eq!(snap.created_at, "<created_at>");
            assert_eq!(snap.deleted_at, Some("<deleted_at>"));
        }
    }

    mod from_schema {
        use crate::helpers::time::string_2_datetime;

        use super::*;

        #[test]
        fn happy_path() {
            let schema = SchemaWortReview {
                wort_id: 1,
                direction: String::from("es_to_de"),
                interval: 2,
                ease_factor: 1.3,
                repetitions: 10,
                last_review: String::from("2018-06-10 20:00:00"),
                next_review: String::from("2018-07-10 20:00:00"),
                created_at: String::from("2020-01-01 20:00:00"),
                deleted_at: None,
            };

            let snap: SnapshotWortReview = schema.into();

            assert_eq!(snap.wort_id, 1);
            assert_eq!(snap.direction, EnumReviewDirection::ES2DE);
            assert_eq!(snap.interval, 2);
            assert_eq!(snap.ease_factor, 1.3);
            assert_eq!(snap.repetitions, 10);
            assert_eq!(
                snap.last_review,
                string_2_datetime("2018-06-10 20:00:00").unwrap()
            );
            assert_eq!(
                snap.next_review,
                string_2_datetime("2018-07-10 20:00:00").unwrap()
            );
            assert_eq!(snap.created_at, "<created_at>");
            assert_eq!(snap.deleted_at, None);
        }

        #[test]
        fn panics_on_invalid_schema() {
            let invalid = SchemaWortReview {
                wort_id: 1,
                direction: String::from("NOT_VALID_DIRECTION"),
                interval: 2,
                ease_factor: 1.3,
                repetitions: 10,
                last_review: String::from("2018-06-10 20:00:00"),
                next_review: String::from("2018-07-10 20:00:00"),
                created_at: String::from("2020-01-01 20:00:00"),
                deleted_at: None,
            };

            // Act + Assert: unwrap() inside From should panic
            let result = std::panic::catch_unwind(|| {
                let _: SnapshotWortReview = invalid.into();
            });

            assert!(
                result.is_err(),
                "expected conversion to panic due to unwrap()"
            );
        }
    }
}
