use chrono::{DateTime, Utc};

use crate::{
    db::schemas::wort_gram_type::schema::SchemaWortGramType,
    helpers::{error_handler::InvalidValueError, time::string_2_datetime},
};

#[derive(Debug, Clone)]
pub struct ModelWortGramType {
    pub id_worte: i32,
    pub id_gram_type: i32,

    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TryFrom<SchemaWortGramType> for ModelWortGramType {
    type Error = Vec<InvalidValueError>;

    fn try_from(value: SchemaWortGramType) -> Result<Self, Self::Error> {
        let mut errs = vec![];

        let id_worte = value.id_worte;
        let id_gram_type = value.id_gram_type;

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
            id_worte,
            id_gram_type,
            created_at: created_at.unwrap(),
            deleted_at,
        })
    }
}

impl ModelWortGramType {
    fn try_from_iter(
        value: impl IntoIterator<Item = SchemaWortGramType>,
    ) -> Result<Vec<ModelWortGramType>, Vec<InvalidValueError>> {
        let mut errs = vec![];
        let mut oks = vec![];

        for v in value {
            match ModelWortGramType::try_from(v) {
                Ok(v) => oks.push(v),
                Err(mut e) => errs.append(&mut e),
            }
        }

        if errs.is_empty() { Ok(oks) } else { Err(errs) }
    }
}

#[cfg(test)]
mod tests_model_wort_gram_type {
    use super::*;
    use color_eyre::Result;

    fn ok_dt() -> &'static str {
        "2025-12-01 00:00:00"
    }

    #[test]
    fn try_from_ok_without_deleted_at() -> Result<()> {
        let schema = SchemaWortGramType {
            id_worte: 10,
            id_gram_type: 4,
            created_at: ok_dt().to_string(),
            deleted_at: None,
        };

        let model = ModelWortGramType::try_from(schema).expect("should be OK");

        assert_eq!(model.id_worte, 10);
        assert_eq!(model.id_gram_type, 4);
        assert_eq!(model.deleted_at, None);

        Ok(())
    }

    #[test]
    fn try_from_ok_with_deleted_at() -> Result<()> {
        let schema = SchemaWortGramType {
            id_worte: 99,
            id_gram_type: 12,
            created_at: ok_dt().to_string(),
            deleted_at: Some("2025-12-31 23:59:59".to_string()),
        };

        let model = ModelWortGramType::try_from(schema).expect("should be OK");

        assert_eq!(model.id_worte, 99);
        assert_eq!(model.id_gram_type, 12);
        assert!(model.deleted_at.is_some());

        Ok(())
    }

    #[test]
    fn try_from_err_invalid_created_at() -> Result<()> {
        let schema = SchemaWortGramType {
            id_worte: 1,
            id_gram_type: 2,
            created_at: "NOT_A_DATE".to_string(),
            deleted_at: None,
        };

        let err = ModelWortGramType::try_from(schema).expect_err("should error");
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].field, "datetime"); // según tu string_2_datetime

        Ok(())
    }

    #[test]
    fn try_from_err_invalid_deleted_at() -> Result<()> {
        let schema = SchemaWortGramType {
            id_worte: 1,
            id_gram_type: 2,
            created_at: ok_dt().to_string(),
            deleted_at: Some("BAD_DATE".to_string()),
        };

        let err = ModelWortGramType::try_from(schema).expect_err("should error");
        assert_eq!(err.len(), 1);
        assert_eq!(err[0].field, "datetime");

        Ok(())
    }

    #[test]
    fn try_from_iter_ok_all_valid() -> Result<()> {
        let data = vec![
            SchemaWortGramType {
                id_worte: 1,
                id_gram_type: 10,
                created_at: ok_dt().to_string(),
                deleted_at: None,
            },
            SchemaWortGramType {
                id_worte: 2,
                id_gram_type: 11,
                created_at: ok_dt().to_string(),
                deleted_at: Some("2025-12-31 23:59:59".to_string()),
            },
        ];

        let out = ModelWortGramType::try_from_iter(data).expect("should be OK");
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].id_worte, 1);
        assert_eq!(out[1].id_worte, 2);

        Ok(())
    }

    #[test]
    fn try_from_iter_err_collects_errors() -> Result<()> {
        let data = vec![
            // OK
            SchemaWortGramType {
                id_worte: 1,
                id_gram_type: 10,
                created_at: ok_dt().to_string(),
                deleted_at: None,
            },
            // BAD created_at
            SchemaWortGramType {
                id_worte: 2,
                id_gram_type: 11,
                created_at: "INVALID".to_string(),
                deleted_at: None,
            },
            // BAD deleted_at
            SchemaWortGramType {
                id_worte: 3,
                id_gram_type: 12,
                created_at: ok_dt().to_string(),
                deleted_at: Some("INVALID2".to_string()),
            },
        ];

        let err = ModelWortGramType::try_from_iter(data).expect_err("should error");
        assert_eq!(err.len(), 2, "should contain both datetime errors");
        assert!(err.iter().all(|e| e.field == "datetime"));

        Ok(())
    }
}
