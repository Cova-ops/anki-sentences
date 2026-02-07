use chrono::{DateTime, Utc};

use crate::db::schemas::setze_review::ModelSetzeReview;

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

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
