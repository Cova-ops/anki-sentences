use chrono::{DateTime, Utc};

use crate::{
    db::schemas::wort_audio::SchemaWortAudio,
    helpers::{error_handler::InvalidValueError, time::string_2_datetime},
};

#[derive(Debug, Clone)]
pub struct ModelWortAudio {
    pub wort_id: i32,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TryFrom<SchemaWortAudio> for ModelWortAudio {
    type Error = Vec<InvalidValueError>;

    fn try_from(value: SchemaWortAudio) -> Result<Self, Self::Error> {
        let mut errs = vec![];

        let wort_id = value.wort_id;
        let audio_name_es = value.audio_name_es;
        let audio_name_de = value.audio_name_de;

        let created_at = match string_2_datetime(value.created_at) {
            Ok(v) => Some(v),
            Err(e) => {
                errs.push(e);
                None
            }
        };

        let deleted_at = if let Some(date) = value.deleted_at {
            match string_2_datetime(date) {
                Ok(v) => Some(v.clone()),
                Err(e) => {
                    errs.push(e);
                    None
                }
            }
        } else {
            None
        };

        if !errs.is_empty() {
            return Err(errs);
        }

        Ok(Self {
            wort_id,
            audio_name_es,
            audio_name_de,
            created_at: created_at.unwrap(),
            deleted_at,
        })
    }
}

impl ModelWortAudio {
    pub fn try_from_iter(
        value: impl IntoIterator<Item = SchemaWortAudio>,
    ) -> Result<Vec<ModelWortAudio>, Vec<InvalidValueError>> {
        let mut errs = vec![];
        let mut oks = vec![];

        for v in value {
            match ModelWortAudio::try_from(v) {
                Ok(v) => oks.push(v),
                Err(mut e) => errs.append(&mut e),
            }
        }

        if errs.is_empty() { Ok(oks) } else { Err(errs) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn schema_ok(deleted_at: Option<&str>) -> SchemaWortAudio {
        SchemaWortAudio {
            wort_id: 10,
            audio_name_es: Some("wort_000010_es.mp3".to_string()),
            audio_name_de: Some("wort_000010_de.mp3".to_string()),
            created_at: "2025-01-01 10:00:00".to_string(),
            deleted_at: deleted_at.map(|s| s.to_string()),
        }
    }

    #[test]
    fn try_from_ok_with_deleted_at() {
        let s = schema_ok(Some("2025-01-02 10:00:00"));

        let m = ModelWortAudio::try_from(s).expect("should convert");

        assert_eq!(m.wort_id, 10);
        assert_eq!(m.audio_name_es.as_deref(), Some("wort_000010_es.mp3"));
        assert_eq!(m.audio_name_de.as_deref(), Some("wort_000010_de.mp3"));

        // si quieres, puedes comparar el string formateado para evitar dudas de tz:
        assert_eq!(m.created_at.to_rfc3339(), "2025-01-01T10:00:00+00:00");
        assert_eq!(
            m.deleted_at.unwrap().to_rfc3339(),
            "2025-01-02T10:00:00+00:00"
        );
    }

    #[test]
    fn try_from_ok_without_deleted_at() {
        let s = schema_ok(None);

        let m = ModelWortAudio::try_from(s).expect("should convert");
        assert_eq!(m.wort_id, 10);
        assert!(m.deleted_at.is_none());
    }

    #[test]
    fn try_from_err_when_created_at_invalid() {
        let mut s = schema_ok(None);
        s.created_at = "NOT_A_DATE".to_string();

        let err = ModelWortAudio::try_from(s).expect_err("should fail");
        assert!(!err.is_empty(), "should return at least one error");

        // opcional: si tu InvalidValueError tiene field/message:
        assert!(err.iter().any(|e| e.field == "datetime"));
    }

    #[test]
    fn try_from_err_when_deleted_at_invalid() {
        let s = schema_ok(Some("NOT_A_DATE"));

        let err = ModelWortAudio::try_from(s).expect_err("should fail");
        assert!(!err.is_empty(), "should return at least one error");
        assert!(err.iter().any(|e| e.field == "datetime"));
    }

    #[test]
    fn try_from_iter_ok_all() {
        let v = vec![schema_ok(None), schema_ok(Some("2025-01-02 10:00:00"))];

        let out = ModelWortAudio::try_from_iter(v).expect("should convert all");
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].wort_id, 10);
        assert_eq!(out[1].wort_id, 10);
    }

    #[test]
    fn try_from_iter_err_accumulates() {
        let mut bad1 = schema_ok(None);
        bad1.created_at = "BAD_CREATED".to_string();

        let bad2 = schema_ok(Some("BAD_DELETED"));

        let v = vec![schema_ok(None), bad1, bad2];

        let err = ModelWortAudio::try_from_iter(v).expect_err("should fail");
        assert!(
            err.len() >= 2,
            "should accumulate errors from multiple items"
        );
        assert!(err.iter().any(|e| e.field == "datetime"));
    }
}
