use chrono::{DateTime, Utc};

use crate::db::schemas::setze_review::{ModelSetzeReview, SchemaSetzeReview};

#[derive(Debug)]
pub struct SnapshotSetzeReview {
    pub id: i32,

    pub satz_id: i32,
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
    pub last_review: DateTime<Utc>,
    pub next_review: DateTime<Utc>,

    // Generic
    pub created_at: &'static str,
    pub deleted_at: Option<&'static str>,
}

impl From<ModelSetzeReview> for SnapshotSetzeReview {
    fn from(value: ModelSetzeReview) -> Self {
        Self {
            id: value.id,

            satz_id: value.satz_id,
            interval: value.interval,
            ease_factor: value.ease_factor,
            repetitions: value.repetitions,
            last_review: value.last_review,
            next_review: value.next_review,

            created_at: "<created_at>",
            deleted_at: value.deleted_at.as_ref().map(|_| "<deleted_at>"),
        }
    }
}

impl From<SchemaSetzeReview> for SnapshotSetzeReview {
    /// Dont use this in prod
    /// It doesnt handle error
    fn from(value: SchemaSetzeReview) -> Self {
        let model = ModelSetzeReview::try_from(value).unwrap();
        model.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

    mod from_model {
        use super::*;

        fn model_ok(deleted: bool) -> ModelSetzeReview {
            ModelSetzeReview {
                id: 1,
                satz_id: 42,
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
            let snap = SnapshotSetzeReview::from(model);

            assert_eq!(snap.id, 1);
            assert_eq!(snap.satz_id, 42);
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
            let snap = SnapshotSetzeReview::from(model);

            assert_eq!(snap.id, 1);
            assert_eq!(snap.satz_id, 42);
            assert_eq!(snap.interval, 4);
            assert_eq!(snap.ease_factor, 2.5);
            assert_eq!(snap.repetitions, 3);

            assert_eq!(snap.created_at, "<created_at>");
            assert_eq!(snap.deleted_at, Some("<deleted_at>"));
        }
    }

    mod from_schema {

        use super::*;

        #[test]
        fn happy_path() {
            let id = 1;
            let satz_id = 10;
            let interval = 1;
            let ease_factor = 1.3;
            let repetitions = 10;
            let last_review = "2026-02-10 20:00:00".to_string();
            let next_review = "2026-02-11 20:00:00".to_string();
            let created_at = "2025-02-10 20:00:00".to_string();
            let deleted_at = None;

            let schema = SchemaSetzeReview {
                id,
                satz_id,
                interval,
                ease_factor,
                repetitions,
                last_review,
                next_review,
                created_at,
                deleted_at,
            };

            let snap: SnapshotSetzeAudio = schema.into();

            assert_eq!(snap.id, id);
            assert_eq!(snap.satz_id, satz_id);
            assert_eq!(snap.interval, interval);
            assert_eq!(snap.ease_factor, ease_factor);
            assert_eq!(snap.repetitions, repetitions);
            assert_eq!(snap.last_review, last_review);
            assert_eq!(snap.next_review, next_review);
            assert_eq!(snap.created_at, created_at);
            assert_eq!(snap.deleted_at, deleted_at);
        }

        #[test]
        fn panics_on_invalid_schema() {
            let id = 1;
            let satz_id = 10;
            let interval = 1;
            let ease_factor = 1.3;
            let repetitions = 10;
            let last_review = "THIS_IS_NOT_A_DATE".to_string();
            let next_review = "2026-02-11 20:00:00".to_string();
            let created_at = "2025-02-10 20:00:00".to_string();
            let deleted_at = None;

            let invalid = SchemaSetzeReview {
                id,
                satz_id,
                interval,
                ease_factor,
                repetitions,
                last_review,
                next_review,
                created_at,
                deleted_at,
            };

            let result = std::panic::catch_unwind(|| {
                let _: SnapshotSetzeAudio = invalid.into();
            });

            assert!(
                result.is_err(),
                "expected conversion to panic due to unwrap()"
            );
        }
    }
}
