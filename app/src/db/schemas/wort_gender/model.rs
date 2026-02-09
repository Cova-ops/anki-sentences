use std::str::FromStr;

use chrono::{DateTime, Utc};

use crate::{
    db::schemas::wort_gender::{EnumWortGender, schema::SchemaWortGender},
    helpers::{error_handler::InvalidValueError, time::string_2_datetime},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelWortGender {
    pub gender: EnumWortGender,

    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TryFrom<SchemaWortGender> for ModelWortGender {
    type Error = Vec<InvalidValueError>;

    fn try_from(value: SchemaWortGender) -> Result<Self, Self::Error> {
        let mut errs = vec![];

        let gender = match EnumWortGender::from_str(&value.gender) {
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

        let deleted_at = if let Some(date) = value.deleted_at {
            match string_2_datetime(date) {
                Ok(v) => Some(v.clone()),
                Err(e) => errs.push(e),
            }
        } else {
            None
        };

        if !errs.is_empty() {
            return Err(errs);
        }

        Ok(Self {
            gender: gender.unwrap(),
            created_at: created_at.unwrap(),
            deleted_at,
        })
    }
}

impl ModelWortGender {
    fn try_from_iter(
        value: impl IntoIterator<Item = SchemaWortGender>,
    ) -> Result<Vec<ModelWortGender>, Vec<InvalidValueError>> {
        let mut errs = vec![];
        let mut oks = vec![];

        for v in value {
            match ModelWortGender::try_from(v) {
                Ok(v) => oks.push(v),
                Err(mut e) => errs.append(&mut e),
            }
        }

        if errs.is_empty() { Ok(oks) } else { Err(errs) }
    }
}

#[cfg(test)]
mod tests_model_wort_gender {
    use super::*;

    fn mk_schema(gender: &str, created_at: &str, deleted_at: Option<&str>) -> SchemaWortGender {
        SchemaWortGender {
            gender: gender.to_string(),
            created_at: created_at.to_string(),
            deleted_at: deleted_at.map(|s| s.to_string()),
        }
    }

    #[test]
    fn try_from_ok_without_deleted_at() {
        let s = mk_schema("maskuline", "2025-12-04 17:44:37", None);

        let m = ModelWortGender::try_from(s).unwrap();

        assert_eq!(m.gender, EnumWortGender::Maskuline);
        assert_eq!(
            m.created_at,
            string_2_datetime("2025-12-04 17:44:37").unwrap()
        );
        assert_eq!(m.deleted_at, None);
    }

    #[test]
    fn try_from_ok_with_deleted_at() {
        let s = mk_schema(
            "femenin",
            "2025-12-04 17:44:37",
            Some("2025-12-05 10:00:00"),
        );

        let m = ModelWortGender::try_from(s).unwrap();

        assert_eq!(m.gender, EnumWortGender::Femenin);
        assert_eq!(
            m.created_at,
            string_2_datetime("2025-12-04 17:44:37").unwrap()
        );
        assert_eq!(
            m.deleted_at,
            Some(string_2_datetime("2025-12-05 10:00:00").unwrap())
        );
    }

    #[test]
    fn try_from_err_invalid_gender() {
        let s = mk_schema("no-existe", "2025-12-04 17:44:37", None);

        let err = ModelWortGender::try_from(s).unwrap_err();

        assert!(!err.is_empty());
        assert!(
            err.iter()
                .any(|e| e.field == "GramType" || e.field == "WortGender")
        );
    }

    #[test]
    fn try_from_err_invalid_created_at() {
        let s = mk_schema("maskuline", "no-date", None);

        let err = ModelWortGender::try_from(s).unwrap_err();

        assert!(err.iter().any(|e| e.field == "datetime"));
        assert!(
            err.iter()
                .any(|e| e.message.contains("Invalid datetime format"))
        );
    }

    #[test]
    fn try_from_err_invalid_deleted_at() {
        let s = mk_schema("maskuline", "2025-12-04 17:44:37", Some("bad-date"));

        let err = ModelWortGender::try_from(s).unwrap_err();

        assert!(err.iter().any(|e| e.field == "datetime"));
        assert!(
            err.iter()
                .any(|e| e.message.contains("Invalid datetime format"))
        );
    }

    #[test]
    fn try_from_err_accumulates_multiple_errors() {
        let s = mk_schema("INVALID", "also-invalid-date", None);

        let err = ModelWortGender::try_from(s).unwrap_err();

        // Debe traer al menos 2 errores: enum + datetime
        assert!(err.len() >= 2);
        assert!(err.iter().any(|e| e.field == "datetime"));
        assert!(
            err.iter()
                .any(|e| e.field == "GramType" || e.field == "WortGender")
        );
    }

    #[test]
    fn try_from_iter_ok_all() {
        let data = vec![
            mk_schema("maskuline", "2025-12-04 17:44:37", None),
            mk_schema(
                "neutrum",
                "2025-12-04 17:44:37",
                Some("2025-12-05 10:00:00"),
            ),
        ];

        let out = ModelWortGender::try_from_iter(data).unwrap();

        assert_eq!(out.len(), 2);
        assert_eq!(out[0].gender, EnumWortGender::Maskuline);
        assert_eq!(out[1].gender, EnumWortGender::Neutrum);
    }

    #[test]
    fn try_from_iter_err_collects_all_errors() {
        let data = vec![
            mk_schema("maskuline", "2025-12-04 17:44:37", None),
            mk_schema("BAD", "bad-date", None), // 2 errores aquí (enum + datetime)
            mk_schema("plural", "2025-12-04 17:44:37", Some("bad-date")), // 1 error (datetime)
        ];

        let err = ModelWortGender::try_from_iter(data).unwrap_err();

        // Debe haber al menos 3 errores (2 + 1)
        assert!(err.len() >= 3);
        assert!(err.iter().any(|e| e.field == "datetime"));
        assert!(
            err.iter()
                .any(|e| e.field == "GramType" || e.field == "WortGender")
        );
    }
}
