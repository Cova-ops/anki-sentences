use std::{collections::HashMap, str::FromStr};

use crate::{
    db::schemas::gram_type::{SchemaGramType, input::InputGramType},
    helpers::error_handler::InvalidValueError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnumGramType {
    NounCommon,
    NounProper,

    VerbMain,
    VerbModal,
    VerbAuxiliary,
    VerbSeparable,
    VerbReflexive,

    Adjective,

    AdverbTime,
    AdverbPlace,
    AdverbManner,
    AdverbDegree,
    AdverbSentenceConnector,

    PronounPersonal,
    PronounPossessive,
    PronounReflexive,
    PronounDemonstrative,
    PronounRelative,
    PronounInterrogative,
    PronounIndefinite,

    ArticleDefinite,
    ArticleIndefinite,

    DeterminerQuantifier,

    PrepositionDative,
    PrepositionAkkusative,
    PrepositionGenitive,
    PrepositionTwoWay,

    ConjunctionCoordinating,
    ConjunctionSubordinating,

    ParticleModal,
    ParticleFocus,
    ParticleNegation,
    ParticleAnswer,

    NumeralCardinal,
    NumeralOrdinal,

    Interjection,
    FixedPhrase,

    PrefixSeparable,

    PatternVerbDativ,
    PatternVerbAkkusativ,
    PatternVerbDatAkk,
}

impl EnumGramType {
    pub const ALL: &[EnumGramType] = &[
        EnumGramType::NounCommon,
        EnumGramType::NounProper,
        EnumGramType::VerbMain,
        EnumGramType::VerbModal,
        EnumGramType::VerbAuxiliary,
        EnumGramType::VerbSeparable,
        EnumGramType::VerbReflexive,
        EnumGramType::Adjective,
        EnumGramType::AdverbTime,
        EnumGramType::AdverbPlace,
        EnumGramType::AdverbManner,
        EnumGramType::AdverbDegree,
        EnumGramType::AdverbSentenceConnector,
        EnumGramType::PronounPersonal,
        EnumGramType::PronounPossessive,
        EnumGramType::PronounReflexive,
        EnumGramType::PronounDemonstrative,
        EnumGramType::PronounRelative,
        EnumGramType::PronounInterrogative,
        EnumGramType::PronounIndefinite,
        EnumGramType::ArticleDefinite,
        EnumGramType::ArticleIndefinite,
        EnumGramType::DeterminerQuantifier,
        EnumGramType::PrepositionDative,
        EnumGramType::PrepositionAkkusative,
        EnumGramType::PrepositionGenitive,
        EnumGramType::PrepositionTwoWay,
        EnumGramType::ConjunctionCoordinating,
        EnumGramType::ConjunctionSubordinating,
        EnumGramType::ParticleModal,
        EnumGramType::ParticleFocus,
        EnumGramType::ParticleNegation,
        EnumGramType::ParticleAnswer,
        EnumGramType::NumeralCardinal,
        EnumGramType::NumeralOrdinal,
        EnumGramType::Interjection,
        EnumGramType::FixedPhrase,
        EnumGramType::PrefixSeparable,
        EnumGramType::PatternVerbDativ,
        EnumGramType::PatternVerbAkkusativ,
        EnumGramType::PatternVerbDatAkk,
    ];

    pub fn to_code(&self) -> &'static str {
        match self {
            EnumGramType::NounCommon => "noun_common",
            EnumGramType::NounProper => "noun_proper",

            EnumGramType::VerbMain => "verb_main",
            EnumGramType::VerbModal => "verb_modal",
            EnumGramType::VerbAuxiliary => "verb_auxiliary",
            EnumGramType::VerbSeparable => "verb_separable",
            EnumGramType::VerbReflexive => "verb_reflexive",

            EnumGramType::Adjective => "adjective",

            EnumGramType::AdverbTime => "adverb_time",
            EnumGramType::AdverbPlace => "adverb_place",
            EnumGramType::AdverbManner => "adverb_manner",
            EnumGramType::AdverbDegree => "adverb_degree",
            EnumGramType::AdverbSentenceConnector => "adverb_sentence_connector",

            EnumGramType::PronounPersonal => "pronoun_personal",
            EnumGramType::PronounPossessive => "pronoun_possessive",
            EnumGramType::PronounReflexive => "pronoun_reflexive",
            EnumGramType::PronounDemonstrative => "pronoun_demonstrative",
            EnumGramType::PronounRelative => "pronoun_relative",
            EnumGramType::PronounInterrogative => "pronoun_interrogative",
            EnumGramType::PronounIndefinite => "pronoun_indefinite",

            EnumGramType::ArticleDefinite => "article_definite",
            EnumGramType::ArticleIndefinite => "article_indefinite",

            EnumGramType::DeterminerQuantifier => "determiner_quantifier",

            EnumGramType::PrepositionDative => "preposition_dative",
            EnumGramType::PrepositionAkkusative => "preposition_akkusative",
            EnumGramType::PrepositionGenitive => "preposition_genitive",
            EnumGramType::PrepositionTwoWay => "preposition_two_way",

            EnumGramType::ConjunctionCoordinating => "conjunction_coordinating",
            EnumGramType::ConjunctionSubordinating => "conjunction_subordinating",

            EnumGramType::ParticleModal => "particle_modal",
            EnumGramType::ParticleFocus => "particle_focus",
            EnumGramType::ParticleNegation => "particle_negation",
            EnumGramType::ParticleAnswer => "particle_answer",

            EnumGramType::NumeralCardinal => "numeral_cardinal",
            EnumGramType::NumeralOrdinal => "numeral_ordinal",

            EnumGramType::Interjection => "interjection",
            EnumGramType::FixedPhrase => "fixed_phrase",

            EnumGramType::PrefixSeparable => "prefix_separable",

            EnumGramType::PatternVerbDativ => "pattern_verb_dativ",
            EnumGramType::PatternVerbAkkusativ => "pattern_verb_akkusativ",
            EnumGramType::PatternVerbDatAkk => "pattern_verb_dat_akk",
        }
    }

    pub fn to_name(&self) -> &'static str {
        match self {
            EnumGramType::NounCommon => "Sustantivo común",
            EnumGramType::NounProper => "Nombre propio",

            EnumGramType::VerbMain => "Verbo léxico",
            EnumGramType::VerbModal => "Verbo modal",
            EnumGramType::VerbAuxiliary => "Verbo auxiliar",
            EnumGramType::VerbSeparable => "Verbo separable",
            EnumGramType::VerbReflexive => "Verbo reflexivo",

            EnumGramType::Adjective => "Adjetivo",

            EnumGramType::AdverbTime => "Adverbio de tiempo",
            EnumGramType::AdverbPlace => "Adverbio de lugar",
            EnumGramType::AdverbManner => "Adverbio de modo",
            EnumGramType::AdverbDegree => "Adverbio de grado",
            EnumGramType::AdverbSentenceConnector => "Adverbio conector",

            EnumGramType::PronounPersonal => "Pronombre personal",
            EnumGramType::PronounPossessive => "Pronombre posesivo",
            EnumGramType::PronounReflexive => "Pronombre reflexivo",
            EnumGramType::PronounDemonstrative => "Pronombre demostrativo",
            EnumGramType::PronounRelative => "Pronombre relativo",
            EnumGramType::PronounInterrogative => "Pronombre interrogativo",
            EnumGramType::PronounIndefinite => "Pronombre indefinido",

            EnumGramType::ArticleDefinite => "Artículo definido",
            EnumGramType::ArticleIndefinite => "Artículo indefinido",

            EnumGramType::DeterminerQuantifier => "Determinante cuantificador",

            EnumGramType::PrepositionDative => "Preposición dativo",
            EnumGramType::PrepositionAkkusative => "Preposición acusativo",
            EnumGramType::PrepositionGenitive => "Preposición genitivo",
            EnumGramType::PrepositionTwoWay => "Preposición de doble vía",

            EnumGramType::ConjunctionCoordinating => "Conjunción coordinante",
            EnumGramType::ConjunctionSubordinating => "Conjunción subordinante",

            EnumGramType::ParticleModal => "Partícula modal",
            EnumGramType::ParticleFocus => "Partícula de enfoque",
            EnumGramType::ParticleNegation => "Partícula de negación",
            EnumGramType::ParticleAnswer => "Partícula de respuesta",

            EnumGramType::NumeralCardinal => "Numeral cardinal",
            EnumGramType::NumeralOrdinal => "Numeral ordinal",

            EnumGramType::Interjection => "Interjección",
            EnumGramType::FixedPhrase => "Frase fija",

            EnumGramType::PrefixSeparable => "Prefijo separable",

            EnumGramType::PatternVerbDativ => "Patrón verbo dativo",
            EnumGramType::PatternVerbAkkusativ => "Patrón verbo acusativo",
            EnumGramType::PatternVerbDatAkk => "Patrón verbo dativo-acusativo",
        }
    }

    pub const fn id(&self) -> i32 {
        match self {
            EnumGramType::NounCommon => 0,
            EnumGramType::NounProper => 1,
            EnumGramType::VerbMain => 2,
            EnumGramType::VerbModal => 3,
            EnumGramType::VerbAuxiliary => 4,
            EnumGramType::VerbSeparable => 5,
            EnumGramType::VerbReflexive => 6,
            EnumGramType::Adjective => 7,
            EnumGramType::AdverbTime => 8,
            EnumGramType::AdverbPlace => 9,
            EnumGramType::AdverbManner => 10,
            EnumGramType::AdverbDegree => 11,
            EnumGramType::AdverbSentenceConnector => 12,
            EnumGramType::PronounPersonal => 13,
            EnumGramType::PronounPossessive => 14,
            EnumGramType::PronounReflexive => 15,
            EnumGramType::PronounDemonstrative => 16,
            EnumGramType::PronounRelative => 17,
            EnumGramType::PronounInterrogative => 18,
            EnumGramType::PronounIndefinite => 19,
            EnumGramType::ArticleDefinite => 20,
            EnumGramType::ArticleIndefinite => 21,
            EnumGramType::DeterminerQuantifier => 22,
            EnumGramType::PrepositionDative => 23,
            EnumGramType::PrepositionAkkusative => 24,
            EnumGramType::PrepositionGenitive => 25,
            EnumGramType::PrepositionTwoWay => 26,
            EnumGramType::ConjunctionCoordinating => 27,
            EnumGramType::ConjunctionSubordinating => 28,
            EnumGramType::ParticleModal => 29,
            EnumGramType::ParticleFocus => 30,
            EnumGramType::ParticleNegation => 31,
            EnumGramType::ParticleAnswer => 32,
            EnumGramType::NumeralCardinal => 33,
            EnumGramType::NumeralOrdinal => 34,
            EnumGramType::Interjection => 35,
            EnumGramType::FixedPhrase => 36,
            EnumGramType::PrefixSeparable => 37,
            EnumGramType::PatternVerbDativ => 38,
            EnumGramType::PatternVerbAkkusativ => 39,
            EnumGramType::PatternVerbDatAkk => 40,
        }
    }

    pub fn to_new(self) -> InputGramType {
        InputGramType { gram: self }
    }

    pub fn get_all_codes() -> Vec<&'static str> {
        Self::ALL.iter().map(|d| d.to_code()).collect()
    }
}

impl FromStr for EnumGramType {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noun_common" => Ok(Self::NounCommon),
            "noun_proper" => Ok(Self::NounProper),

            "verb_main" => Ok(Self::VerbMain),
            "verb_modal" => Ok(Self::VerbModal),
            "verb_auxiliary" => Ok(Self::VerbAuxiliary),
            "verb_separable" => Ok(Self::VerbSeparable),
            "verb_reflexive" => Ok(Self::VerbReflexive),

            "adjective" => Ok(Self::Adjective),

            "adverb_time" => Ok(Self::AdverbTime),
            "adverb_place" => Ok(Self::AdverbPlace),
            "adverb_manner" => Ok(Self::AdverbManner),
            "adverb_degree" => Ok(Self::AdverbDegree),
            "adverb_sentence_connector" => Ok(Self::AdverbSentenceConnector),

            "pronoun_personal" => Ok(Self::PronounPersonal),
            "pronoun_possessive" => Ok(Self::PronounPossessive),
            "pronoun_reflexive" => Ok(Self::PronounReflexive),
            "pronoun_demonstrative" => Ok(Self::PronounDemonstrative),
            "pronoun_relative" => Ok(Self::PronounRelative),
            "pronoun_interrogative" => Ok(Self::PronounInterrogative),
            "pronoun_indefinite" => Ok(Self::PronounIndefinite),

            "article_definite" => Ok(Self::ArticleDefinite),
            "article_indefinite" => Ok(Self::ArticleIndefinite),

            "determiner_quantifier" => Ok(Self::DeterminerQuantifier),

            "preposition_dative" => Ok(Self::PrepositionDative),
            "preposition_akkusative" => Ok(Self::PrepositionAkkusative),
            "preposition_genitive" => Ok(Self::PrepositionGenitive),
            "preposition_two_way" => Ok(Self::PrepositionTwoWay),

            "conjunction_coordinating" => Ok(Self::ConjunctionCoordinating),
            "conjunction_subordinating" => Ok(Self::ConjunctionSubordinating),

            "particle_modal" => Ok(Self::ParticleModal),
            "particle_focus" => Ok(Self::ParticleFocus),
            "particle_negation" => Ok(Self::ParticleNegation),
            "particle_answer" => Ok(Self::ParticleAnswer),

            "numeral_cardinal" => Ok(Self::NumeralCardinal),
            "numeral_ordinal" => Ok(Self::NumeralOrdinal),

            "interjection" => Ok(Self::Interjection),
            "fixed_phrase" => Ok(Self::FixedPhrase),

            "prefix_separable" => Ok(Self::PrefixSeparable),

            "pattern_verb_dativ" => Ok(Self::PatternVerbDativ),
            "pattern_verb_akkusativ" => Ok(Self::PatternVerbAkkusativ),
            "pattern_verb_dat_akk" => Ok(Self::PatternVerbDatAkk),

            _ => Err(InvalidValueError {
                field: "GramType",
                message: format!("{s} is not a GramType valid"),
                valid_options: Some(Self::get_all_codes()),
            }),
        }
    }
}

impl TryFrom<SchemaGramType> for EnumGramType {
    type Error = InvalidValueError;

    fn try_from(value: SchemaGramType) -> Result<Self, Self::Error> {
        Self::from_str(&value.code)
    }
}

impl TryFrom<i32> for EnumGramType {
    type Error = InvalidValueError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let mut ids: HashMap<i32, Self> =
            Self::ALL.iter().map(|d| (d.id(), d.to_owned())).collect();

        match ids.remove(&value) {
            Some(v) => Ok(v),
            _ => Err(InvalidValueError {
                field: "GramType",
                message: format!("{value} is not a valid id for GramType"),
                valid_options: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::str::FromStr;

    mod all {
        use super::*;

        #[test]
        fn all_has_expected_len() {
            assert_eq!(EnumGramType::ALL.len(), 41);
        }

        #[test]
        fn all_has_no_duplicates() {
            let set: HashSet<EnumGramType> = EnumGramType::ALL.iter().copied().collect();
            assert_eq!(set.len(), EnumGramType::ALL.len());
        }

        #[test]
        fn ids_cover_0_to_40_without_gaps() {
            let mut ids: Vec<i32> = EnumGramType::ALL.iter().map(|g| g.id()).collect();
            ids.sort_unstable();
            ids.dedup();

            let expected: Vec<i32> = (0..=40).collect();
            assert_eq!(ids, expected);
        }
    }

    mod to_code {
        use super::*;

        #[test]
        fn to_code_is_unique() {
            let mut codes: Vec<&'static str> =
                EnumGramType::ALL.iter().map(|g| g.to_code()).collect();
            codes.sort_unstable();
            codes.dedup();
            assert_eq!(codes.len(), EnumGramType::ALL.len());
        }

        #[test]
        fn to_code_and_from_str_round_trip_for_all() {
            for g in EnumGramType::ALL {
                let code = g.to_code();
                let parsed = EnumGramType::from_str(code)
                    .unwrap_or_else(|_| panic!("failed parsing code={code}"));
                assert_eq!(parsed, *g);
            }
        }
    }

    mod to_name {
        use super::*;

        #[test]
        fn to_name_is_not_empty_for_all() {
            for g in EnumGramType::ALL {
                let name = g.to_name();
                assert!(!name.trim().is_empty(), "empty name for {:?}", g);
            }
        }
    }

    mod to_new {
        use super::*;

        #[test]
        fn to_new_builds_insert_struct_correctly_for_all() {
            for g in EnumGramType::ALL {
                let new = g.to_new();

                assert_eq!(new.gram.id(), g.id());
                assert_eq!(new.gram.to_code(), g.to_code());
                assert_eq!(new.gram.to_name(), g.to_name());
            }
        }
    }

    mod get_all_codes {
        use super::*;

        #[test]
        fn get_all_codes_matches_all_iter() {
            let mut a = EnumGramType::get_all_codes();
            let mut b: Vec<String> = EnumGramType::ALL
                .iter()
                .map(|g| g.to_code().to_string())
                .collect();

            a.sort();
            b.sort();
            assert_eq!(a, b);
        }
    }

    mod from_str {
        use super::*;

        #[test]
        fn from_str_and_to_code_round_trip_for_all_codes() {
            for code in EnumGramType::get_all_codes() {
                let parsed = EnumGramType::from_str(&code)
                    .unwrap_or_else(|_| panic!("failed parsing code={code}"));
                assert_eq!(parsed.to_code(), code);
            }
        }

        #[test]
        fn from_str_invalid_returns_err() {
            let err = EnumGramType::from_str("not_a_real_gram_type").unwrap_err();

            assert_eq!(err.field, "GramType");
            assert!(err.message.contains("not_a_real_gram_type"));
            assert!(err.valid_options.as_ref().unwrap().len() == EnumGramType::ALL.len());
        }
    }

    mod try_from_schema {
        use super::*;

        #[test]
        fn try_from_schema_ok() {
            let raw = SchemaGramType {
                code: "verb_modal".to_string(),
                created_at: "2025-12-01 10:00:00".to_string(),
                deleted_at: None,
            };

            let out = EnumGramType::try_from(raw).unwrap();
            assert_eq!(out, EnumGramType::VerbModal);
        }

        #[test]
        fn try_from_schema_err_invalid_code() {
            let raw = SchemaGramType {
                code: "no_existe".to_string(),
                created_at: "2025-12-01 10:00:00".to_string(),
                deleted_at: None,
            };

            let err: InvalidValueError = EnumGramType::try_from(raw).unwrap_err();

            assert_eq!(err.field, "GramType");
            assert_eq!(err.message, "no_existe is not a GramType valid");
            assert!(err.valid_options.is_some());
            assert!(
                err.valid_options
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|s| *s == "verb_modal")
            );
        }
    }

    mod try_from_i32 {
        use super::*;

        #[test]
        fn try_from_returns_ok_for_all_known_ids() {
            for g in EnumGramType::ALL.iter() {
                let id = g.id();
                let out = EnumGramType::try_from(id)
                    .unwrap_or_else(|e| panic!("expected Ok for id={id}, got Err: {e:?}"));

                assert_eq!(
                    out, *g,
                    "EnumGramType::try_from(id) should return the exact variant for id={id}"
                );
            }
        }

        #[test]
        fn try_from_returns_err_for_unknown_id() {
            // pick an id that is guaranteed not to exist in ALL
            let max_id = EnumGramType::ALL.iter().map(|g| g.id()).max().unwrap_or(0);

            let invalid = max_id + 1;

            let err = EnumGramType::try_from(invalid).unwrap_err();

            assert_eq!(err.field, "GramType");
            assert!(
                err.message.contains("is not a valid id for GramType"),
                "unexpected message: {}",
                err.message
            );
            assert!(err.message.contains(&invalid.to_string()));
            assert!(err.valid_options.is_none());
        }

        #[test]
        fn try_from_is_deterministic() {
            // same input should always map to same output
            let sample = EnumGramType::ALL.first().unwrap().id();

            let a = EnumGramType::try_from(sample).unwrap();
            let b = EnumGramType::try_from(sample).unwrap();

            assert_eq!(a, b);
        }
    }
}
