use chrono::{DateTime, Utc};

use crate::{
    db::schemas::{
        gram_type::{EnumGramType, SchemaGramType},
        niveau_liste::EnumNiveauListe,
        wort::SchemaWort,
        wort_gender::EnumWortGender,
    },
    helpers::{error_handler::InvalidValueError, time::string_2_datetime},
};

#[derive(Debug, Clone)]
pub struct ModelWort {
    pub id: i32,
    pub gram_type: Vec<EnumGramType>,
    pub gender: Option<EnumWortGender>,
    pub worte_de: String,
    pub worte_es: String,
    pub plural: Option<String>,
    pub niveau: EnumNiveauListe,
    pub example_de: String,
    pub example_es: String,

    // nur verben
    pub verb_aux: Option<String>,
    pub trennbar: Option<bool>,
    pub reflexiv: Option<bool>,

    // Generic
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl TryFrom<(SchemaWort, Vec<SchemaGramType>)> for ModelWort {
    type Error = Vec<InvalidValueError>;

    fn try_from((wort, grams): (SchemaWort, Vec<SchemaGramType>)) -> Result<Self, Self::Error> {
        let mut errs: Vec<InvalidValueError> = vec![];

        // gram types
        let mut gram_type: Vec<EnumGramType> = Vec::with_capacity(grams.len());
        for g in grams.into_iter() {
            match EnumGramType::try_from(g) {
                Ok(v) => gram_type.push(v),
                Err(e) => errs.push(e),
            }
        }

        // gender
        let gender: Option<EnumWortGender> = match wort.gender_id {
            Some(id) => match EnumWortGender::try_from(id) {
                Ok(v) => Some(v),
                Err(e) => {
                    errs.push(e);
                    None
                }
            },
            None => None,
        };

        // niveau
        let niveau: Option<EnumNiveauListe> = match EnumNiveauListe::try_from(wort.niveau_id) {
            Ok(v) => Some(v),
            Err(e) => {
                errs.push(e);
                None
            }
        };

        // created/deleted
        let created_at = match string_2_datetime(&wort.created_at) {
            Ok(v) => Some(v),
            Err(e) => {
                errs.push(e);
                None
            }
        };

        let deleted_at = match wort.deleted_at.as_deref() {
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
            id: wort.id,
            gram_type,
            gender,
            worte_de: wort.worte_de,
            worte_es: wort.worte_es,
            plural: wort.plural,
            niveau: niveau.unwrap(),
            example_de: wort.example_de,
            example_es: wort.example_es,
            verb_aux: wort.verb_aux,
            trennbar: wort.trennbar,
            reflexiv: wort.reflexiv,
            created_at: created_at.unwrap(),
            deleted_at,
        })
    }
}

impl ModelWort {
    pub fn try_from_iter(
        value: impl IntoIterator<Item = (SchemaWort, Vec<SchemaGramType>)>,
    ) -> Result<Vec<ModelWort>, Vec<InvalidValueError>> {
        let mut errs = vec![];
        let mut oks = vec![];

        for v in value {
            match ModelWort::try_from(v) {
                Ok(v) => oks.push(v),
                Err(mut e) => errs.append(&mut e),
            }
        }

        if errs.is_empty() { Ok(oks) } else { Err(errs) }
    }
}

#[cfg(test)]
mod tests_model_wort {
    use super::*;
    use chrono::Utc;

    fn schema_wort_base() -> SchemaWort {
        SchemaWort {
            id: 1,
            gender_id: Some(0),
            worte_de: "Hund".into(),
            worte_es: "Perro".into(),
            plural: Some("Hunde".into()),
            niveau_id: 1,
            example_de: "Der Hund ist da.".into(),
            example_es: "El perro está ahí.".into(),
            verb_aux: None,
            trennbar: None,
            reflexiv: None,
            created_at: "2025-12-09 20:30:00".into(),
            deleted_at: None,
        }
    }

    fn grams_ok() -> Vec<SchemaGramType> {
        vec![
            SchemaGramType {
                code: "noun_common".into(),
                created_at: "2025-12-09 20:30:00".into(),
                deleted_at: None,
            },
            SchemaGramType {
                code: "adjective".into(),
                created_at: "2025-12-09 20:30:00".into(),
                deleted_at: None,
            },
        ]
    }

    #[test]
    fn try_from_ok() -> Result<(), Box<dyn std::error::Error>> {
        let wort = schema_wort_base();
        let grams = grams_ok();

        let model = ModelWort::try_from((wort, grams)).expect("should build");

        assert_eq!(model.id, 1);
        assert_eq!(model.worte_de, "Hund");
        assert_eq!(model.worte_es, "Perro");
        assert_eq!(model.plural.as_deref(), Some("Hunde"));
        assert_eq!(model.gender, Some(EnumWortGender::Maskuline)); // id 0
        assert_eq!(model.niveau, EnumNiveauListe::A2); // id 1
        assert_eq!(model.gram_type.len(), 2);
        assert_eq!(model.created_at.timezone(), &Utc);
        assert!(model.deleted_at.is_none());

        Ok(())
    }

    #[test]
    fn try_from_accumulates_errors() {
        let mut wort = schema_wort_base();
        wort.gender_id = Some(999); // invalid
        wort.niveau_id = 999; // invalid
        wort.created_at = "not-a-date".into(); // invalid
        wort.deleted_at = Some("also-not-a-date".into()); // invalid

        let mut grams = grams_ok();
        grams.push(SchemaGramType {
            code: "not_a_gram_type".into(), // invalid
            created_at: "2025-12-09 20:30:00".into(),
            deleted_at: None,
        });

        let err = ModelWort::try_from((wort, grams)).unwrap_err();

        // Debe traer varios errores (al menos 5 por lo de arriba)
        assert!(err.len() >= 5, "expected multiple errors, got: {:#?}", err);

        // Opcional: checar que vienen de campos esperados (depende tu InvalidValueError)
        let fields: Vec<_> = err.iter().map(|e| e.field).collect();
        assert!(
            fields.contains(&"WortGender")
                || fields.contains(&"GramType")
                || fields.contains(&"NiveauListe")
                || fields.contains(&"datetime")
        );
    }

    #[test]
    fn try_from_iter_ok_all() {
        let data = vec![
            (schema_wort_base(), grams_ok()),
            (
                {
                    let mut w = schema_wort_base();
                    w.id = 2;
                    w.worte_de = "Katze".into();
                    w.worte_es = "Gato".into();
                    w
                },
                grams_ok(),
            ),
        ];

        let out = ModelWort::try_from_iter(data).expect("should pass");
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].id, 1);
        assert_eq!(out[1].id, 2);
    }

    #[test]
    fn try_from_iter_collects_errors_from_multiple_items() {
        let ok = (schema_wort_base(), grams_ok());

        let mut bad_wort = schema_wort_base();
        bad_wort.id = 99;
        bad_wort.niveau_id = 999;
        bad_wort.created_at = "bad".into();

        let mut bad_grams = grams_ok();
        bad_grams.push(SchemaGramType {
            code: "bad_gram".into(),
            created_at: "2025-12-09 20:30:00".into(),
            deleted_at: None,
        });

        let bad = (bad_wort, bad_grams);

        let err = ModelWort::try_from_iter(vec![ok, bad]).unwrap_err();

        assert!(!err.is_empty(), "expected errors");
    }
}
