use std::str::FromStr;

use chrono::{DateTime, Utc};

use crate::{
    db::schemas::gram_type::{SchemaGramType, enums::EnumGramType},
    helpers::{error_handler::InvalidValueError, time::string_2_datetime},
};

#[derive(Debug, Clone)]
pub struct ModelGramType {
    pub gram: EnumGramType,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TryFrom<SchemaGramType> for ModelGramType {
    type Error = Vec<InvalidValueError>;

    fn try_from(value: SchemaGramType) -> Result<Self, Self::Error> {
        let mut errs = vec![];

        let gram = match EnumGramType::from_str(&value.code) {
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
            gram: gram.unwrap(),
            created_at: created_at.unwrap(),
            deleted_at,
        })
    }
}

impl ModelGramType {
    pub fn try_from_iter(
        value: impl IntoIterator<Item = SchemaGramType>,
    ) -> Result<Vec<ModelGramType>, Vec<InvalidValueError>> {
        let mut errs = vec![];
        let mut oks = vec![];

        for v in value {
            match ModelGramType::try_from(v) {
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

    fn schema(code: &str, created_at: &str, deleted_at: Option<&str>) -> SchemaGramType {
        SchemaGramType {
            code: code.to_owned(),
            created_at: created_at.to_owned(),
            deleted_at: deleted_at.map(|s| s.to_owned()),
        }
    }

    // Formato SQLite: "YYYY-MM-DD HH:MM:SS"
    const OK_DT: &str = "2025-12-04 17:44:37";

    #[test]
    fn try_from_ok_without_deleted_at() -> Result<(), Vec<InvalidValueError>> {
        let raw = schema("noun_common", OK_DT, None);
        let m = ModelGramType::try_from(&raw)?;

        assert_eq!(m.gram, EnumGramType::NounCommon);
        assert!(m.deleted_at.is_none());

        Ok(())
    }

    #[test]
    fn try_from_ok_with_deleted_at() -> Result<(), Vec<InvalidValueError>> {
        let raw = schema("verb_auxiliary", OK_DT, Some("2025-12-05 10:00:00"));
        let m = ModelGramType::try_from(&raw)?;

        assert_eq!(m.gram, EnumGramType::VerbAuxiliary);
        assert!(m.deleted_at.is_some());

        Ok(())
    }

    #[test]
    fn try_from_err_invalid_code_only() {
        let raw = schema("not_a_real_code", OK_DT, None);
        let err = ModelGramType::try_from(&raw).unwrap_err();

        assert_eq!(err.len(), 1);
    }

    #[test]
    fn try_from_err_invalid_created_at_only() {
        let raw = schema("noun_common", "NOT_A_DATE", None);
        let err = ModelGramType::try_from(&raw).unwrap_err();

        assert_eq!(err.len(), 1);
    }

    #[test]
    fn try_from_err_invalid_deleted_at_only() {
        let raw = schema("noun_common", OK_DT, Some("NOT_A_DATE"));
        let err = ModelGramType::try_from(&raw).unwrap_err();

        assert_eq!(err.len(), 1);
    }

    #[test]
    fn try_from_err_multiple_fields() {
        // code inválido + created_at inválido + deleted_at inválido
        let raw = schema("bad_code", "BAD_DATE", Some("BAD_DATE_TOO"));
        let err = ModelGramType::try_from(&raw).unwrap_err();

        assert_eq!(err.len(), 3);
    }

    #[test]
    fn try_from_iter_ok_all() -> Result<(), Vec<InvalidValueError>> {
        let data = vec![
            schema("noun_common", OK_DT, None),
            schema("noun_proper", OK_DT, Some("2025-12-05 10:00:00")),
            schema("verb_main", OK_DT, None),
        ];

        let out = ModelGramType::try_from_iter(data.iter())?;
        assert_eq!(out.len(), 3);

        assert_eq!(out[0].gram, EnumGramType::NounCommon);
        assert_eq!(out[1].gram, EnumGramType::NounProper);
        assert_eq!(out[2].gram, EnumGramType::VerbMain);

        Ok(())
    }

    #[test]
    fn try_from_iter_err_accumulates() {
        let data = vec![
            schema("noun_common", OK_DT, None),        // ok
            schema("bad_code", OK_DT, None),           // 1 err (code)
            schema("verb_main", "BAD_DATE", None),     // 1 err (created_at)
            schema("noun_proper", OK_DT, Some("BAD")), // 1 err (deleted_at)
            schema("bad", "BAD", Some("BAD")),         // 3 err
        ];

        let err = ModelGramType::try_from_iter(data.iter()).unwrap_err();

        // 1 + 1 + 1 + 3 = 6 errores
        assert_eq!(err.len(), 6);
    }

    #[test]
    fn try_from_iter_err_returns_err_not_partial_ok() {
        let data = vec![
            schema("noun_common", OK_DT, None),
            schema("bad_code", OK_DT, None),
        ];

        let res = ModelGramType::try_from_iter(data.iter());
        assert!(res.is_err());
    }
}
