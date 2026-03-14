use std::str::FromStr;

use crate::{
    helpers::error_handler::InvalidValueError, services::tts::language_voice::LanguageVoice,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnumReviewDirection {
    ES2DE,
    DE2ES,
}

impl EnumReviewDirection {
    pub const ALL: &[Self] = &[Self::ES2DE, Self::DE2ES];

    pub fn as_str(&self) -> &'static str {
        match self {
            EnumReviewDirection::ES2DE => "es_to_de",
            EnumReviewDirection::DE2ES => "de_to_es",
        }
    }

    pub fn get_all_str() -> Vec<&'static str> {
        Self::ALL.iter().map(|d| d.as_str()).collect()
    }
}

impl From<LanguageVoice> for EnumReviewDirection {
    fn from(value: LanguageVoice) -> Self {
        match value {
            LanguageVoice::Spanisch => Self::ES2DE,
            LanguageVoice::Deutsch => Self::DE2ES,
        }
    }
}

impl FromStr for EnumReviewDirection {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "es_to_de" => Ok(EnumReviewDirection::ES2DE),
            "de_to_es" => Ok(EnumReviewDirection::DE2ES),

            _ => Err(InvalidValueError {
                field: "EnumReviewDirection",
                message: format!("{s} is not a EnumReviewDirection valid"),
                valid_options: Some(Self::get_all_str()),
            }),
        }
    }
}

#[cfg(test)]
mod tests_review_direction {
    use super::*;

    #[test]
    fn as_str_matches_expected() {
        assert_eq!(EnumReviewDirection::ES2DE.as_str(), "es_to_de");
        assert_eq!(EnumReviewDirection::DE2ES.as_str(), "de_to_es");
    }

    #[test]
    fn get_all_str_contains_all_values_in_order() {
        let all = EnumReviewDirection::get_all_str();
        assert_eq!(all, vec!["es_to_de", "de_to_es"]);
    }

    #[test]
    fn from_language_voice_maps_correctly() {
        assert_eq!(
            EnumReviewDirection::from(LanguageVoice::Spanisch),
            EnumReviewDirection::ES2DE
        );
        assert_eq!(
            EnumReviewDirection::from(LanguageVoice::Deutsch),
            EnumReviewDirection::DE2ES
        );
    }

    #[test]
    fn from_str_parses_valid_values() {
        assert_eq!(
            EnumReviewDirection::from_str("es_to_de").unwrap(),
            EnumReviewDirection::ES2DE
        );
        assert_eq!(
            EnumReviewDirection::from_str("de_to_es").unwrap(),
            EnumReviewDirection::DE2ES
        );
    }

    #[test]
    fn from_str_invalid_returns_invalid_value_error_with_options() {
        let err = EnumReviewDirection::from_str("xx_to_yy").unwrap_err();

        assert_eq!(err.field, "EnumReviewDirection");
        assert_eq!(err.message, "xx_to_yy is not a EnumReviewDirection valid");

        // valid_options: Some(Vec<&'static str>)
        assert_eq!(
            err.valid_options.as_deref(),
            Some(&["es_to_de", "de_to_es"][..])
        );
    }
}
