use chrono::{DateTime, Utc};

use crate::{
    db::schemas::{niveau_liste::EnumNiveauListe, setze::SchemaSetze},
    helpers::{error_handler::InvalidValueError, time::string_2_datetime},
};

#[derive(Debug, Clone)]
pub struct ModelSetze {
    pub id: i32,

    pub setze_spanisch: String,
    pub setze_deutsch: String,
    pub niveau: EnumNiveauListe,
    pub thema: String,

    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TryFrom<SchemaSetze> for ModelSetze {
    type Error = Vec<InvalidValueError>;

    fn try_from(value: SchemaSetze) -> Result<Self, Self::Error> {
        let mut errs = vec![];

        let niveau = match EnumNiveauListe::try_from(value.niveau_id) {
            Ok(v) => Some(v),
            Err(e) => {
                errs.push(e);
                None
            }
        };

        let created_at = match string_2_datetime(value.created_at) {
            Ok(v) => Some(v),
            Err(e) => {
                errs.push(e);
                None
            }
        };

        let deleted_at = match value.deleted_at.as_deref() {
            Some(s) => match string_2_datetime(s) {
                Ok(v) => Some(v),
                Err(e) => {
                    errs.push(e);
                    None
                }
            },
            None => None,
        };

        if !errs.is_empty() {
            return Err(errs);
        }

        Ok(Self {
            id: value.id,
            setze_spanisch: value.setze_spanisch,
            setze_deutsch: value.setze_deutsch,
            niveau: niveau.unwrap(),
            thema: value.thema,
            created_at: created_at.unwrap(),
            deleted_at,
        })
    }
}

impl ModelSetze {
    pub fn try_from_iter(
        value: impl IntoIterator<Item = SchemaSetze>,
    ) -> Result<Vec<ModelSetze>, Vec<InvalidValueError>> {
        let mut errs = vec![];
        let mut oks = vec![];

        for v in value {
            match ModelSetze::try_from(v) {
                Ok(v) => oks.push(v),
                Err(mut e) => errs.append(&mut e),
            }
        }

        if errs.is_empty() { Ok(oks) } else { Err(errs) }
    }
}

#[cfg(test)]
mod tests_model_setze {
    use super::*;

    fn schema_ok(deleted_at: Option<&str>) -> SchemaSetze {
        SchemaSetze {
            id: 1,
            setze_spanisch: "Hola".to_string(),
            setze_deutsch: "Hallo".to_string(),
            niveau_id: 0, // A1
            thema: "Saludos".to_string(),
            created_at: "2025-12-09 20:30:00".to_string(),
            deleted_at: deleted_at.map(|s| s.to_string()),
        }
    }

    #[test]
    fn try_from_ok_without_deleted_at() {
        let s = schema_ok(None);
        let m = ModelSetze::try_from(s).expect("should build ModelSetze");

        assert_eq!(m.id, 1);
        assert_eq!(m.setze_spanisch, "Hola");
        assert_eq!(m.setze_deutsch, "Hallo");
        assert_eq!(m.niveau, EnumNiveauListe::A1);
        assert_eq!(m.thema, "Saludos");

        // sanity: created_at parsed
        assert_eq!(m.created_at.to_rfc3339(), "2025-12-09T20:30:00+00:00");
        assert!(m.deleted_at.is_none());
    }

    #[test]
    fn try_from_ok_with_deleted_at() {
        let s = schema_ok(Some("2025-12-10 10:00:00"));
        let m = ModelSetze::try_from(s).expect("should build ModelSetze");

        assert!(m.deleted_at.is_some());
        assert_eq!(
            m.deleted_at.unwrap().to_rfc3339(),
            "2025-12-10T10:00:00+00:00"
        );
    }

    #[test]
    fn try_from_err_invalid_niveau_id() {
        let mut s = schema_ok(None);
        s.niveau_id = 999;

        let err = ModelSetze::try_from(s).unwrap_err();
        assert!(!err.is_empty());
        assert!(err.iter().any(|e| e.field == "NiveauListe"));
    }

    #[test]
    fn try_from_err_invalid_created_at() {
        let mut s = schema_ok(None);
        s.created_at = "not-a-date".to_string();

        let err = ModelSetze::try_from(s).unwrap_err();
        assert!(!err.is_empty());
        assert!(err.iter().any(|e| e.field == "datetime"));
    }

    #[test]
    fn try_from_err_invalid_deleted_at() {
        let mut s = schema_ok(Some("2025-12-10 10:00:00"));
        s.deleted_at = Some("bad-date".to_string());

        let err = ModelSetze::try_from(s).unwrap_err();
        assert!(!err.is_empty());
        assert!(err.iter().any(|e| e.field == "datetime"));
    }

    #[test]
    fn try_from_err_accumulates_multiple_errors() {
        let mut s = schema_ok(Some("bad-date"));
        s.niveau_id = 999;
        s.created_at = "also-bad".to_string();

        let err = ModelSetze::try_from(s).unwrap_err();
        // should include niveau + created_at + deleted_at
        assert!(err.len() >= 3);
        assert!(err.iter().any(|e| e.field == "NiveauListe"));
        assert!(err.iter().filter(|e| e.field == "datetime").count() >= 2);
    }

    #[test]
    fn try_from_iter_ok() {
        let v = vec![schema_ok(None), schema_ok(Some("2025-12-10 10:00:00"))];
        let out = ModelSetze::try_from_iter(v).expect("should build vec of models");
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].niveau, EnumNiveauListe::A1);
        assert!(out[0].deleted_at.is_none());
        assert!(out[1].deleted_at.is_some());
    }

    #[test]
    fn try_from_iter_err_accumulates_all_errors() {
        let mut a = schema_ok(None);
        a.niveau_id = 999;

        let mut b = schema_ok(None);
        b.created_at = "bad".to_string();

        let err = ModelSetze::try_from_iter(vec![a, b]).unwrap_err();
        assert!(err.len() >= 2);
        assert!(err.iter().any(|e| e.field == "NiveauListe"));
        assert!(err.iter().any(|e| e.field == "datetime"));
    }
}
