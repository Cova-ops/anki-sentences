use crate::db::schemas::{
    niveau_liste::EnumNiveauListe,
    setze::{ModelSetze, SchemaSetze},
};

#[derive(Debug, Clone)]
pub struct SnapshotSetze {
    pub id: i32,

    pub setze_spanisch: String,
    pub setze_deutsch: String,
    pub niveau: EnumNiveauListe,
    pub thema: String,

    pub created_at: &'static str,
    pub deleted_at: Option<&'static str>,
}

impl From<ModelSetze> for SnapshotSetze {
    fn from(value: ModelSetze) -> Self {
        Self {
            id: value.id,

            setze_spanisch: value.setze_spanisch,
            setze_deutsch: value.setze_deutsch,
            niveau: value.niveau,
            thema: value.thema,

            created_at: "<created_at>",
            deleted_at: value.deleted_at.as_ref().map(|_| "<deleted_at>"),
        }
    }
}

impl From<SchemaSetze> for SnapshotSetze {
    /// Don't use this in production
    /// It doesn't handle errors
    fn from(value: SchemaSetze) -> Self {
        let model = ModelSetze::try_from(value).unwrap();
        model.into()
    }
}

#[cfg(test)]
mod tests_snapshot_setze {
    use super::*;

    use chrono::{DateTime, Utc};

    mod from_model {
        use super::*;

        fn model_setze(deleted_at: Option<DateTime<Utc>>) -> ModelSetze {
            ModelSetze {
                id: 10,
                setze_spanisch: "Hola".to_string(),
                setze_deutsch: "Hallo".to_string(),
                niveau: EnumNiveauListe::A1,
                thema: "Saludos".to_string(),
                created_at: DateTime::<Utc>::from_timestamp(1_700_000_000, 0).unwrap(),
                deleted_at,
            }
        }

        #[test]
        fn from_model_without_deleted_at() {
            let m = model_setze(None);
            let snap = SnapshotSetze::from(m);

            assert_eq!(snap.id, 10);
            assert_eq!(snap.setze_spanisch, "Hola");
            assert_eq!(snap.setze_deutsch, "Hallo");
            assert_eq!(snap.niveau, EnumNiveauListe::A1);
            assert_eq!(snap.thema, "Saludos");

            assert_eq!(snap.created_at, "<created_at>");
            assert!(snap.deleted_at.is_none());
        }

        #[test]
        fn from_model_with_deleted_at() {
            let m = model_setze(Some(
                DateTime::<Utc>::from_timestamp(1_700_000_100, 0).unwrap(),
            ));
            let snap = SnapshotSetze::from(m);

            assert_eq!(snap.created_at, "<created_at>");
            assert_eq!(snap.deleted_at, Some("<deleted_at>"));
        }
    }

    mod from_schema {
        use super::*;

        #[test]
        fn happy_path() {
            let schema = SchemaSetze {
                id: 1,
                setze_spanisch: "Estoy aprendiendo alemán.".to_string(),
                setze_deutsch: "Ich lerne Deutsch.".to_string(),
                niveau_id: 0,
                thema: "learning".to_string(),
                created_at: "2026-02-10 10:00:00".to_string(),
                deleted_at: None,
            };

            let snap: SnapshotSetze = schema.into();

            assert_eq!(snap.id, 1);
            assert_eq!(snap.setze_spanisch, "Estoy aprendiendo alemán.");
            assert_eq!(snap.setze_deutsch, "Ich lerne Deutsch.");
            assert_eq!(snap.niveau, EnumNiveauListe::A1);
            assert_eq!(snap.thema, "learning");
        }

        #[test]
        fn panics_on_invalid_schema() {
            // Arrange: build a SchemaSetzeAudio that will make ModelSetzeAudio::try_from fail
            // Common failure: empty file_path, invalid voice_id, etc.
            let invalid = SchemaSetze {
                id: 1,
                setze_spanisch: "Estoy aprendiendo alemán.".to_string(),
                setze_deutsch: "Ich lerne Deutsch.".to_string(),
                niveau_id: -1,
                thema: "learning".to_string(),
                created_at: "THIS_IS_NOT_A_DATE".to_string(),
                deleted_at: None,
            };

            // Act + Assert: because your impl does unwrap(), this should panic
            let result = std::panic::catch_unwind(|| {
                let _: SnapshotSetze = invalid.into();
            });

            assert!(
                result.is_err(),
                "expected conversion to panic due to unwrap()"
            );
        }
    }
}
