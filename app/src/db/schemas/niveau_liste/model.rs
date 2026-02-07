use std::str::FromStr;

use chrono::{DateTime, Utc};

use crate::{
    db::schemas::niveau_liste::{EnumNiveauListe, SchemaNiveauListe},
    helpers::{error_handler::InvalidValueError, time::string_2_datetime},
};

pub struct ModelNiveauListe {
    pub niveau: EnumNiveauListe,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TryFrom<SchemaNiveauListe> for ModelNiveauListe {
    type Error = Vec<InvalidValueError>;

    fn try_from(value: SchemaNiveauListe) -> Result<Self, Self::Error> {
        let mut errs = vec![];

        let niveau = match EnumNiveauListe::from_str(&value.niveau) {
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
            niveau: niveau.unwrap(),
            created_at: created_at.unwrap(),
            deleted_at,
        })
    }
}

impl ModelNiveauListe {
    fn try_from_iter(
        value: impl IntoIterator<Item = SchemaNiveauListe>,
    ) -> Result<Vec<ModelNiveauListe>, Vec<InvalidValueError>> {
        let mut errs = vec![];
        let mut oks = vec![];

        for v in value {
            match ModelNiveauListe::try_from(v) {
                Ok(v) => oks.push(v),
                Err(mut e) => errs.append(&mut e),
            }
        }

        if errs.is_empty() { Ok(oks) } else { Err(errs) }
    }
}

#[cfg(test)]
mod tests_model_niveau_liste {
    use super::*;

    fn schema(niveau: &str, created_at: &str, deleted_at: Option<&str>) -> SchemaNiveauListe {
        SchemaNiveauListe {
            niveau: niveau.to_string(),
            created_at: created_at.to_string(),
            deleted_at: deleted_at.map(|s| s.to_string()),
        }
    }

    #[test]
    fn try_from_ok_deleted_at_none() {
        let s = schema("A2", "2025-12-04 18:07:37", None);

        let m = ModelNiveauListe::try_from(s).unwrap();

        assert_eq!(m.niveau, EnumNiveauListe::A2);
        assert_eq!(m.created_at.to_rfc3339(), "2025-12-04T18:07:37+00:00");
        assert!(m.deleted_at.is_none());
    }

    #[test]
    fn try_from_ok_deleted_at_some_sqlite_format() {
        let s = schema("B1", "2025-12-04 18:07:37", Some("2025-12-31 00:00:00"));

        let m = ModelNiveauListe::try_from(s).unwrap();

        assert_eq!(m.niveau, EnumNiveauListe::B1);
        assert_eq!(m.created_at.to_rfc3339(), "2025-12-04T18:07:37+00:00");
        assert_eq!(
            m.deleted_at.unwrap().to_rfc3339(),
            "2025-12-31T00:00:00+00:00"
        );
    }

    #[test]
    fn try_from_err_accumulates_multiple_errors() {
        // niveau inválido + created_at inválido + deleted_at inválido => 3 errores
        let s = schema("Z9", "not-a-date", Some("also-bad"));

        let errs = ModelNiveauListe::try_from(s).unwrap_err();

        assert_eq!(errs.len(), 3);

        // opcional: checar campos (depende tu InvalidValueError)
        assert!(
            errs.iter()
                .any(|e| e.field == "NiveauListe" || e.field == "niveau")
        );
        assert!(errs.iter().any(|e| e.field == "datetime"));
    }

    #[test]
    fn try_from_err_only_niveau_invalid() {
        let s = schema("Z9", "2025-12-04 18:07:37", None);

        let errs = ModelNiveauListe::try_from(s).unwrap_err();

        assert_eq!(errs.len(), 1);
        assert!(errs[0].field == "NiveauListe" || errs[0].field == "niveau");
    }

    #[test]
    fn try_from_err_only_created_at_invalid() {
        let s = schema("A1", "bad-date", None);

        let errs = ModelNiveauListe::try_from(s).unwrap_err();

        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].field, "datetime");
    }

    #[test]
    fn try_from_err_only_deleted_at_invalid() {
        let s = schema("A1", "2025-12-04 18:07:37", Some("bad-date"));

        let errs = ModelNiveauListe::try_from(s).unwrap_err();

        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].field, "datetime");
    }

    #[test]
    fn string_2_datetime_accepts_sqlite_format() {
        let dt = string_2_datetime("2025-12-04 18:07:37").unwrap();
        assert_eq!(dt.to_rfc3339(), "2025-12-04T18:07:37+00:00");
    }

    #[test]
    fn string_2_datetime_accepts_fractional_with_offset() {
        let dt = string_2_datetime("2025-12-04 17:44:37.548062+00:00").unwrap();
        // lo convertimos a UTC, debe quedar igual en +00:00
        assert!(dt.to_rfc3339().starts_with("2025-12-04T17:44:37.548062"));
        assert!(dt.to_rfc3339().ends_with("+00:00"));
    }

    #[test]
    fn string_2_datetime_accepts_rfc3339() {
        let dt = string_2_datetime("2025-12-04T17:44:37.548062Z").unwrap();
        assert!(dt.to_rfc3339().starts_with("2025-12-04T17:44:37.548062"));
        assert!(dt.to_rfc3339().ends_with("+00:00"));
    }

    #[test]
    fn string_2_datetime_rejects_invalid() {
        let err = string_2_datetime("nope").unwrap_err();
        assert_eq!(err.field, "datetime");
        assert!(err.message.contains("Invalid datetime format"));
        assert!(err.valid_options.is_some());
    }

    #[test]
    fn try_from_iter_ok_all() {
        let input = vec![
            schema("A1", "2025-12-04 18:07:37", None),
            schema("A2", "2025-12-04 18:07:38", Some("2025-12-31 00:00:00")),
        ];

        let out = ModelNiveauListe::try_from_iter(input).unwrap();
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].niveau, EnumNiveauListe::A1);
        assert_eq!(out[1].niveau, EnumNiveauListe::A2);
    }

    #[test]
    fn try_from_iter_err_if_any_item_fails_and_accumulates() {
        let input = vec![
            schema("A1", "2025-12-04 18:07:37", None),  // ok
            schema("Z9", "bad-date", Some("also-bad")), // 3 errs
            schema("A2", "bad-date", None),             // 1 err
        ];

        let errs = ModelNiveauListe::try_from_iter(input).unwrap_err();
        assert_eq!(errs.len(), 4);
    }

    #[test]
    fn try_from_iter_err_all_fail() {
        let input = vec![
            schema("Z9", "bad", None),       // 2 errs (niveau+created_at)
            schema("X", "bad", Some("bad")), // 3 errs
        ];

        let errs = ModelNiveauListe::try_from_iter(input).unwrap_err();
        assert_eq!(errs.len(), 5);
    }
}
