use crate::db::schemas::gram_type::{EnumGramType, ModelGramType};

#[derive(Debug, Clone)]
pub struct SnapshotGramType {
    pub gram: EnumGramType,
    pub created_at: &'static str,
    pub deleted_at: Option<&'static str>,
}

impl From<ModelGramType> for SnapshotGramType {
    fn from(value: ModelGramType) -> Self {
        Self {
            gram: value.gram,
            created_at: "<created_at>",
            deleted_at: value.deleted_at.as_ref().map(|_| "<deleted_at>"),
        }
    }
}

#[cfg(test)]
mod tests_snapshot_gram_type {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_snapshot_gram_type() {
        let model = ModelGramType {
            gram: EnumGramType::VerbMain,
            created_at: Utc::now(),
            deleted_at: Some(Utc::now()),
        };

        let snap = SnapshotGramType::from(model);

        assert_eq!(snap.gram, EnumGramType::VerbMain);
        assert_eq!(snap.created_at, "<created_at>");
        assert_eq!(snap.deleted_at, Some("<deleted_at>"));
    }

    #[test]
    fn test_snapshot_gram_type_deleted_at_none() {
        let model = ModelGramType {
            gram: EnumGramType::VerbMain,
            created_at: Utc::now(),
            deleted_at: None,
        };

        let snap = SnapshotGramType::from(model);

        assert_eq!(snap.gram, EnumGramType::VerbMain);
        assert_eq!(snap.created_at, "<created_at>");
        assert_eq!(snap.deleted_at, None);
    }
}
