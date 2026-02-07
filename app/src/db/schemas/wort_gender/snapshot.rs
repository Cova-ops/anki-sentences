use crate::db::schemas::wort_gender::{EnumWortGender, ModelWortGender};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotWortGender {
    pub gender: EnumWortGender,

    pub created_at: &'static str,
    pub deleted_at: Option<&'static str>,
}

impl From<ModelWortGender> for SnapshotWortGender {
    fn from(value: ModelWortGender) -> Self {
        Self {
            gender: value.gender,
            created_at: "<created_at>",
            deleted_at: value.deleted_at.as_ref().map(|_| "<deleted_at>"),
        }
    }
}

#[cfg(test)]
mod tests_snapshot_wort_gender {
    use super::*;
    use chrono::{DateTime, Utc};

    fn mk_model(gender: EnumWortGender, deleted_at: Option<DateTime<Utc>>) -> ModelWortGender {
        ModelWortGender {
            gender,
            created_at: Utc::now(),
            deleted_at,
        }
    }

    #[test]
    fn snapshot_from_model_without_deleted_at() {
        let model = mk_model(EnumWortGender::Maskuline, None);

        let snap = SnapshotWortGender::from(model);

        assert_eq!(snap.gender, EnumWortGender::Maskuline);
        assert_eq!(snap.created_at, "<created_at>");
        assert_eq!(snap.deleted_at, None);
    }

    #[test]
    fn snapshot_from_model_with_deleted_at() {
        let model = mk_model(EnumWortGender::Femenin, Some(Utc::now()));

        let snap = SnapshotWortGender::from(model);

        assert_eq!(snap.gender, EnumWortGender::Femenin);
        assert_eq!(snap.created_at, "<created_at>");
        assert_eq!(snap.deleted_at, Some("<deleted_at>"));
    }

    #[test]
    fn snapshot_preserves_gender_correctly() {
        for gender in EnumWortGender::ALL {
            let model = mk_model(*gender, None);
            let snap = SnapshotWortGender::from(model);

            assert_eq!(snap.gender, *gender);
        }
    }
}
