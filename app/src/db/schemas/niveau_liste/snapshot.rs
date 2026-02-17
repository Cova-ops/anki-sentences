use crate::db::schemas::niveau_liste::{EnumNiveauListe, ModelNiveauListe, SchemaNiveauListe};

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

impl From<SchemaNiveauListe> for SnapshotNiveauListe {
    /// Don't use this on production
    /// It doesn't handle errors
    fn from(value: SchemaNiveauListe) -> Self {
        let model = ModelNiveauListe::try_from(value).unwrap();
        model.into()
    }
}

#[cfg(test)]
mod tests_snapshot_niveau_liste {
    use crate::db::schemas::niveau_liste::{
        EnumNiveauListe, ModelNiveauListe, SchemaNiveauListe, SnapshotNiveauListe,
    };
    use chrono::{TimeZone, Utc};

    mod from_model {

        use super::*;

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

    mod from_schema {
        use std::str::FromStr;

        use super::*;

        #[test]
        fn from_schema_to_snapshot_ok() {
            let schema = SchemaNiveauListe {
                niveau: "A2".to_string(),
                created_at: "2025-12-04 18:07:37".to_string(),
                deleted_at: None,
            };

            let snap: SnapshotNiveauListe = schema.into();

            assert_eq!(snap.niveau, EnumNiveauListe::from_str("A2").unwrap());
            // si tu snapshot pone placeholders en fechas, ajusta esto
            // por ejemplo:
            // assert_eq!(snap.created_at, "<created_at>");
            // assert_eq!(snap.deleted_at, "<deleted_at>");
        }

        #[test]
        #[should_panic]
        fn from_schema_to_snapshot_panics_if_schema_is_invalid_for_model() {
            // Ejemplo de input inválido para ModelNiveauListe::try_from(...)
            // Ajusta este caso a la validación real que tengas.
            let schema = SchemaNiveauListe {
                niveau: "NOT_A_LEVEL".to_string(),
                created_at: "2025-12-04 18:07:37".to_string(),
                deleted_at: None,
            };

            let _: SnapshotNiveauListe = schema.into(); // debe panic por unwrap()
        }
    }
}
