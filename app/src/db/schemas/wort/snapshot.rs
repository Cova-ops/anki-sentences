use crate::db::schemas::{
    gram_type::EnumGramType,
    niveau_liste::EnumNiveauListe,
    wort::{ModelWort, SchemaWort},
    wort_gender::EnumWortGender,
    wort_gram_type::SchemaWortGramType,
};

#[derive(Debug, Clone)]
pub struct SnapshotWort {
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
    pub created_at: &'static str,
    pub deleted_at: Option<&'static str>,
}

impl From<ModelWort> for SnapshotWort {
    fn from(value: ModelWort) -> Self {
        Self {
            id: value.id,
            gram_type: value.gram_type,
            gender: value.gender,
            worte_de: value.worte_de,
            worte_es: value.worte_es,
            plural: value.plural,
            niveau: value.niveau,
            example_de: value.example_de,
            example_es: value.example_es,

            // nur verben
            verb_aux: value.verb_aux,
            trennbar: value.trennbar,
            reflexiv: value.reflexiv,

            created_at: "<created_at>",
            deleted_at: value.deleted_at.as_ref().map(|_| "<deleted_at>"),
        }
    }
}

impl From<SchemaWort> for SnapshotWort {
    /// Don't use this on prod
    /// It doesn't handle errors
    fn from(value: SchemaWort) -> Self {
        let model = ModelWort::try_from((value, vec![])).unwrap();
        model.into()
    }
}

impl From<(SchemaWort, Vec<SchemaWortGramType>)> for SnapshotWort {
    /// Don't use this on prod
    /// It doesn't handle errors
    fn from((wort, wort_gt): (SchemaWort, Vec<SchemaWortGramType>)) -> Self {
        let model = ModelWort::try_from((wort, wort_gt)).unwrap();
        model.into()
    }
}

#[cfg(test)]
mod tests_snapshot_wort {
    use chrono::Utc;

    use super::*;

    mod from_model {
        use super::*;

        #[test]
        fn snapshot_from_model_without_deleted_at() {
            let model = ModelWort {
                id: 1,
                gram_type: vec![EnumGramType::NounCommon, EnumGramType::Adjective],
                gender: Some(EnumWortGender::Maskuline),
                worte_de: "Hund".into(),
                worte_es: "Perro".into(),
                plural: Some("Hunde".into()),
                niveau: EnumNiveauListe::A2,
                example_de: "Der Hund ist da.".into(),
                example_es: "El perro está ahí.".into(),
                verb_aux: None,
                trennbar: Some(false),
                reflexiv: Some(false),
                created_at: Utc::now(),
                deleted_at: None,
            };

            let snap = SnapshotWort::from(model);

            assert_eq!(snap.id, 1);
            assert_eq!(
                snap.gram_type,
                vec![EnumGramType::NounCommon, EnumGramType::Adjective]
            );
            assert_eq!(snap.gender, Some(EnumWortGender::Maskuline));
            assert_eq!(snap.worte_de, "Hund");
            assert_eq!(snap.worte_es, "Perro");
            assert_eq!(snap.plural.as_deref(), Some("Hunde"));
            assert_eq!(snap.niveau, EnumNiveauListe::A2);
            assert_eq!(snap.example_de, "Der Hund ist da.");
            assert_eq!(snap.example_es, "El perro está ahí.");
            assert_eq!(snap.verb_aux, None);
            assert_eq!(snap.trennbar, Some(false));
            assert_eq!(snap.reflexiv, Some(false));

            assert_eq!(snap.created_at, "<created_at>");
            assert_eq!(snap.deleted_at, None);
        }

        #[test]
        fn snapshot_from_model_with_deleted_at() {
            let model = ModelWort {
                id: 2,
                gram_type: vec![EnumGramType::VerbMain],
                gender: None,
                worte_de: "gehen".into(),
                worte_es: "ir".into(),
                plural: None,
                niveau: EnumNiveauListe::A1,
                example_de: "Ich gehe nach Hause.".into(),
                example_es: "Voy a casa.".into(),
                verb_aux: Some("sein".into()),
                trennbar: Some(false),
                reflexiv: Some(false),
                created_at: Utc::now(),
                deleted_at: Some(Utc::now()),
            };

            let snap = SnapshotWort::from(model);

            assert_eq!(snap.id, 2);
            assert_eq!(snap.gram_type, vec![EnumGramType::VerbMain]);
            assert_eq!(snap.gender, None);
            assert_eq!(snap.worte_de, "gehen");
            assert_eq!(snap.worte_es, "ir");
            assert_eq!(snap.plural, None);
            assert_eq!(snap.niveau, EnumNiveauListe::A1);
            assert_eq!(snap.example_de, "Ich gehe nach Hause.");
            assert_eq!(snap.example_es, "Voy a casa.");
            assert_eq!(snap.verb_aux.as_deref(), Some("sein"));
            assert_eq!(snap.trennbar, Some(false));
            assert_eq!(snap.reflexiv, Some(false));

            assert_eq!(snap.created_at, "<created_at>");
            assert_eq!(snap.deleted_at, Some("<deleted_at>"));
        }
    }

    mod from_schema {
        use super::*;

        #[test]
        fn happy_path() {
            let schema = SchemaWort {
                id: 1,
                gender_id: Some(1), // Femenin
                worte_de: "Hund".into(),
                worte_es: "Perro".into(),
                plural: Some("Hunde".into()),
                niveau_id: 2,
                example_de: "Der Hund ist da.".into(),
                example_es: "El perro está ahí.".into(),
                verb_aux: None,
                trennbar: Some(false),
                reflexiv: Some(false),
                created_at: String::from("2019-07-03 20:00:00"),
                deleted_at: None,
            };

            let snap: SnapshotWort = schema.into();

            assert_eq!(snap.id, 1);
            assert_eq!(snap.gram_type, vec![]);
            assert_eq!(snap.gender, Some(EnumWortGender::Femenin));
            assert_eq!(snap.worte_de, "Hund");
            assert_eq!(snap.worte_es, "Perro");
            assert_eq!(snap.plural.as_deref(), Some("Hunde"));
            assert_eq!(snap.niveau, EnumNiveauListe::B1);
            assert_eq!(snap.example_de, "Der Hund ist da.");
            assert_eq!(snap.example_es, "El perro está ahí.");
            assert_eq!(snap.verb_aux, None);
            assert_eq!(snap.trennbar, Some(false));
            assert_eq!(snap.reflexiv, Some(false));

            assert_eq!(snap.created_at, "<created_at>");
            assert_eq!(snap.deleted_at, None);
        }

        #[test]
        fn panics_on_invalid_schema() {
            // Arrange: build a SchemaSetzeAudio that will make ModelSetzeAudio::try_from fail
            // Common failure: empty file_path, invalid voice_id, etc.
            let invalid = SchemaWort {
                id: 1,
                gender_id: Some(999), // Not exists
                worte_de: "Hund".into(),
                worte_es: "Perro".into(),
                plural: Some("Hunde".into()),
                niveau_id: -99, // Invalid
                example_de: "Der Hund ist da.".into(),
                example_es: "El perro está ahí.".into(),
                verb_aux: None,
                trennbar: Some(false),
                reflexiv: Some(false),
                created_at: String::from("2019-07-03 20:00:00"),
                deleted_at: None,
            };

            // Act + Assert: because your impl does unwrap(), this should panic
            let result = std::panic::catch_unwind(|| {
                let _: SnapshotWort = invalid.into();
            });

            assert!(
                result.is_err(),
                "expected conversion to panic due to unwrap()"
            );
        }
    }
}
