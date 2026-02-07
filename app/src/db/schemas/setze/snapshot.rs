use crate::db::schemas::{niveau_liste::EnumNiveauListe, setze::ModelSetze};

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

#[cfg(test)]
mod tests_snapshot_setze {
    use super::*;
    use chrono::{DateTime, Utc};

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
