use crate::db::schemas::gram_type::{EnumGramType, ModelGramType, SchemaGramType};

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

impl From<SchemaGramType> for SnapshotGramType {
    /// Don't use this for production, only for testing
    /// It doesn't handle errors
    fn from(value: SchemaGramType) -> Self {
        let model = ModelGramType::try_from(value).unwrap();
        SnapshotGramType::from(model)
    }
}

#[cfg(test)]
mod tests_snapshot_gram_type {
    use super::*;
    use chrono::Utc;

    mod from_model {
        use super::*;

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

    mod from_schema {
        use super::*;

        #[test]
        fn from_schema_maps_fields_and_placeholders() {
            // GIVEN: a SchemaGramType that can be converted into ModelGramType
            let schema = SchemaGramType {
                code: EnumGramType::VerbMain.to_code().to_string(),
                created_at: "2026-02-01T00:00:00Z".to_string(),
                deleted_at: Some("2026-02-02T00:00:00Z".to_string()),
            };

            // WHEN
            let snap: SnapshotGramType = schema.into();

            // THEN: the Snapshot conversion should placeholder dates and keep the enum value
            assert_eq!(snap.gram, EnumGramType::VerbMain);
            assert_eq!(snap.created_at, "<created_at>");
            assert_eq!(snap.deleted_at, Some("<deleted_at>"));
        }

        #[test]
        #[should_panic]
        fn from_schema_panics_if_schema_is_invalid_for_model() {
            // This conversion uses unwrap() intentionally, so invalid input should panic.
            // Pick an id that EnumGramType / ModelGramType rejects.
            let schema = SchemaGramType {
                code: "bad_code".to_string(),
                created_at: "2026-02-01T00:00:00Z".to_string(),
                deleted_at: None,
            };

            let _snap: SnapshotGramType = schema.into();
        }
    }
}
