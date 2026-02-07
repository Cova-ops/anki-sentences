use crate::db::schemas::wort_gram_type::ModelWortGramType;

#[derive(Debug)]
pub struct SnapshotWortGramType {
    pub id_worte: i32,
    pub id_gram_type: i32,

    pub created_at: &'static str,
    pub deleted_at: Option<&'static str>,
}

impl From<ModelWortGramType> for SnapshotWortGramType {
    fn from(value: ModelWortGramType) -> Self {
        Self {
            id_worte: value.id_worte,
            id_gram_type: value.id_gram_type,

            created_at: "<created_at>",
            deleted_at: value.deleted_at.as_ref().map(|_| "<deleted_at>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};

    fn model_ok(deleted: bool) -> ModelWortGramType {
        ModelWortGramType {
            id_worte: 10,
            id_gram_type: 3,
            created_at: DateTime::<Utc>::from_timestamp(1_700_000_000, 0).unwrap(),
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
        let snap = SnapshotWortGramType::from(model);

        assert_eq!(snap.id_worte, 10);
        assert_eq!(snap.id_gram_type, 3);

        assert_eq!(snap.created_at, "<created_at>");
        assert!(snap.deleted_at.is_none());
    }

    #[test]
    fn snapshot_from_model_with_deleted_at() {
        let model = model_ok(true);
        let snap = SnapshotWortGramType::from(model);

        assert_eq!(snap.id_worte, 10);
        assert_eq!(snap.id_gram_type, 3);

        assert_eq!(snap.created_at, "<created_at>");
        assert_eq!(snap.deleted_at, Some("<deleted_at>"));
    }
}
