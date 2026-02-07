use crate::db::schemas::niveau_liste::{EnumNiveauListe, ModelNiveauListe};

#[derive(Debug, Clone)]
pub struct SnapshotNiveauListe {
    pub niveau: EnumNiveauListe,
    pub created_at: &'static str,
    pub deleted_at: Option<&'static str>,
}

impl From<ModelNiveauListe> for SnapshotNiveauListe {
    fn from(value: ModelNiveauListe) -> Self {
        Self {
            niveau: value.niveau,
            created_at: "<created_at>",
            deleted_at: value.deleted_at.as_ref().map(|_| "<deleted_at>"),
        }
    }
}

#[cfg(test)]
mod tests_snapshot_niveau_liste {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn snapshot_from_model_without_deleted_at() {
        let model = ModelNiveauListe {
            niveau: EnumNiveauListe::A2,
            created_at: Utc.with_ymd_and_hms(2025, 12, 4, 18, 7, 37).unwrap(),
            deleted_at: None,
        };

        let snap = SnapshotNiveauListe::from(model);

        assert_eq!(snap.niveau, EnumNiveauListe::A2);
        assert_eq!(snap.created_at, "<created_at>");
        assert_eq!(snap.deleted_at, None);
    }

    #[test]
    fn snapshot_from_model_with_deleted_at() {
        let model = ModelNiveauListe {
            niveau: EnumNiveauListe::B1,
            created_at: Utc.with_ymd_and_hms(2025, 12, 4, 18, 7, 37).unwrap(),
            deleted_at: Some(Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap()),
        };

        let snap = SnapshotNiveauListe::from(model);

        assert_eq!(snap.niveau, EnumNiveauListe::B1);
        assert_eq!(snap.created_at, "<created_at>");
        assert_eq!(snap.deleted_at, Some("<deleted_at>"));
    }
}
