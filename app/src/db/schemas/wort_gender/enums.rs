use std::str::FromStr;

use crate::{db::schemas::wort_gender::InputWortGender, helpers::error_handler::InvalidValueError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnumWortGender {
    Maskuline,
    Femenin,
    Neutrum,
    Plural,
}

impl EnumWortGender {
    pub const ALL: &[Self] = &[Self::Maskuline, Self::Femenin, Self::Neutrum, Self::Plural];

    pub fn id(&self) -> i32 {
        match self {
            Self::Maskuline => 0,
            Self::Femenin => 1,
            Self::Neutrum => 2,
            Self::Plural => 3,
        }
    }

    pub fn gender(&self) -> &'static str {
        match self {
            Self::Maskuline => "Maskuline",
            Self::Femenin => "Femenin",
            Self::Neutrum => "Neutrum",
            Self::Plural => "Plural",
        }
    }

    pub fn artikel(&self) -> &'static str {
        match self {
            Self::Maskuline => "der",
            Self::Femenin => "die",
            Self::Neutrum => "das",
            Self::Plural => "die",
        }
    }

    pub fn get_all_genders() -> Vec<&'static str> {
        Self::ALL.iter().map(|d| d.gender()).collect()
    }

    pub fn to_new(&self) -> InputWortGender {
        InputWortGender {
            gender: self.to_owned(),
        }
    }
}

impl FromStr for EnumWortGender {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "maskuline" => Ok(Self::Maskuline),
            "femenin" => Ok(Self::Femenin),
            "neutrum" => Ok(Self::Neutrum),
            "plural" => Ok(Self::Plural),

            _ => Err(InvalidValueError {
                field: "WortGender",
                message: format!("{s} is not a WortGender valid"),
                valid_options: Some(Self::get_all_genders()),
            }),
        }
    }
}

impl TryFrom<i32> for EnumWortGender {
    type Error = InvalidValueError;

    fn try_from(id: i32) -> Result<Self, Self::Error> {
        match id {
            0 => Ok(Self::Maskuline),
            1 => Ok(Self::Femenin),
            2 => Ok(Self::Neutrum),
            3 => Ok(Self::Plural),

            _ => Err(InvalidValueError {
                field: "WortGender",
                message: format!("{id} is not an id valid for WortGender"),
                valid_options: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests_enum_worte_gender {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn all_contains_every_variant_once() {
        let all = EnumWortGender::ALL;

        assert_eq!(all.len(), 4);

        let set: HashSet<EnumWortGender> = all.iter().copied().collect();
        assert_eq!(set.len(), 4);

        assert!(set.contains(&EnumWortGender::Maskuline));
        assert!(set.contains(&EnumWortGender::Femenin));
        assert!(set.contains(&EnumWortGender::Neutrum));
        assert!(set.contains(&EnumWortGender::Plural));
    }

    #[test]
    fn id_is_stable_and_unique() {
        assert_eq!(EnumWortGender::Maskuline.id(), 0);
        assert_eq!(EnumWortGender::Femenin.id(), 1);
        assert_eq!(EnumWortGender::Neutrum.id(), 2);
        assert_eq!(EnumWortGender::Plural.id(), 3);

        let ids: HashSet<i32> = EnumWortGender::ALL.iter().map(|g| g.id()).collect();
        assert_eq!(ids.len(), EnumWortGender::ALL.len());
    }

    #[test]
    fn gender_labels() {
        assert_eq!(EnumWortGender::Maskuline.gender(), "Maskuline");
        assert_eq!(EnumWortGender::Femenin.gender(), "Femenin");
        assert_eq!(EnumWortGender::Neutrum.gender(), "Neutrum");
        assert_eq!(EnumWortGender::Plural.gender(), "Plural");
    }

    #[test]
    fn artikel_labels() {
        assert_eq!(EnumWortGender::Maskuline.artikel(), "der");
        assert_eq!(EnumWortGender::Femenin.artikel(), "die");
        assert_eq!(EnumWortGender::Neutrum.artikel(), "das");
        assert_eq!(EnumWortGender::Plural.artikel(), "die");
    }

    #[test]
    fn to_new_maps_fields_correctly() {
        let g = EnumWortGender::Neutrum;
        let new = g.to_new();

        assert_eq!(new.gender.id(), 2);
        assert_eq!(new.gender.gender(), "Neutrum");
        assert_eq!(new.gender.artikel(), "das");
    }

    #[test]
    fn from_str_accepts_case_insensitive_inputs() {
        assert_eq!(
            EnumWortGender::from_str("Maskuline").unwrap(),
            EnumWortGender::Maskuline
        );
        assert_eq!(
            EnumWortGender::from_str("maskuline").unwrap(),
            EnumWortGender::Maskuline
        );
        assert_eq!(
            EnumWortGender::from_str("FEMENIN").unwrap(),
            EnumWortGender::Femenin
        );
        assert_eq!(
            EnumWortGender::from_str("NeUtRuM").unwrap(),
            EnumWortGender::Neutrum
        );
        assert_eq!(
            EnumWortGender::from_str("plural").unwrap(),
            EnumWortGender::Plural
        );
    }

    #[test]
    fn from_str_invalid_returns_invalid_value_error_with_options() {
        let err = EnumWortGender::from_str("nope").unwrap_err();

        // Ajusta esto si cambiaste el field/message en tu InvalidValueError.
        assert_eq!(err.field, "WortGender");
        assert!(err.message.contains("nope"));

        assert!(err.valid_options.is_some());
        let opts = err.valid_options.unwrap();
        assert!(!opts.is_empty());

        // Si tu get_all_codes() devuelve ["maskuline","femenin","neutrum","plural"], valida eso:
        // (Si devuelve otra cosa, ajusta estas aserciones)
        assert!(opts.iter().any(|s| *s == "Maskuline"));
        assert!(opts.iter().any(|s| *s == "Femenin"));
        assert!(opts.iter().any(|s| *s == "Neutrum"));
        assert!(opts.iter().any(|s| *s == "Plural"));
    }

    #[test]
    fn try_from_i32_valid_values() {
        assert_eq!(
            EnumWortGender::try_from(0).unwrap(),
            EnumWortGender::Maskuline
        );
        assert_eq!(
            EnumWortGender::try_from(1).unwrap(),
            EnumWortGender::Femenin
        );
        assert_eq!(
            EnumWortGender::try_from(2).unwrap(),
            EnumWortGender::Neutrum
        );
        assert_eq!(EnumWortGender::try_from(3).unwrap(), EnumWortGender::Plural);
    }

    #[test]
    fn try_from_i32_invalid_value_negative() {
        let err: InvalidValueError = EnumWortGender::try_from(-1).unwrap_err();

        assert_eq!(err.field, "WortGender");
        assert_eq!(err.message, "-1 is not an id valid for WortGender");
        assert_eq!(err.valid_options, None);
    }

    #[test]
    fn try_from_i32_invalid_value_out_of_range() {
        let err: InvalidValueError = EnumWortGender::try_from(99).unwrap_err();

        assert_eq!(err.field, "WortGender");
        assert_eq!(err.message, "99 is not an id valid for WortGender");
        assert_eq!(err.valid_options, None);
    }
}
