use std::str::FromStr;

use crate::{
    db::schemas::niveau_liste::InputNiveauListe, helpers::error_handler::InvalidValueError,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumNiveauListe {
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
}

impl EnumNiveauListe {
    pub const ALL: &[Self] = &[Self::A1, Self::A2, Self::B1, Self::B2, Self::C1, Self::C2];

    pub const fn id(&self) -> i32 {
        match self {
            Self::A1 => 0,
            Self::A2 => 1,
            Self::B1 => 2,
            Self::B2 => 3,
            Self::C1 => 4,
            Self::C2 => 5,
        }
    }

    pub fn to_new(&self) -> InputNiveauListe {
        InputNiveauListe {
            niveau: self.clone(),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::A1 => "A1",
            Self::A2 => "A2",
            Self::B1 => "B1",
            Self::B2 => "B2",
            Self::C1 => "C1",
            Self::C2 => "C2",
        }
    }
}

impl FromStr for EnumNiveauListe {
    type Err = InvalidValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A1" => Ok(Self::A1),
            "A2" => Ok(Self::A2),
            "B1" => Ok(Self::B1),
            "B2" => Ok(Self::B2),
            "C1" => Ok(Self::C1),
            "C2" => Ok(Self::C2),

            _ => Err(InvalidValueError {
                field: "NiveauListe",
                message: format!("{s} is not a NiveauListe valid"),
                valid_options: Some(Self::ALL.iter().map(|d| d.as_str()).collect()),
            }),
        }
    }
}

impl TryFrom<i32> for EnumNiveauListe {
    type Error = InvalidValueError;

    fn try_from(id: i32) -> Result<Self, Self::Error> {
        match id {
            0 => Ok(Self::A1),
            1 => Ok(Self::A2),
            2 => Ok(Self::B1),
            3 => Ok(Self::B2),
            4 => Ok(Self::C1),
            5 => Ok(Self::C2),

            _ => Err(InvalidValueError {
                field: "NiveauListe",
                message: format!("{id} is not a valid id for NiveauListe"),
                valid_options: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_all_contains_all_variants_in_order() {
        let all = EnumNiveauListe::ALL;

        assert_eq!(all.len(), 6);
        assert_eq!(
            all,
            &[
                EnumNiveauListe::A1,
                EnumNiveauListe::A2,
                EnumNiveauListe::B1,
                EnumNiveauListe::B2,
                EnumNiveauListe::C1,
                EnumNiveauListe::C2,
            ]
        );
    }

    #[test]
    fn test_id_mapping() {
        let cases = vec![
            (EnumNiveauListe::A1, 0),
            (EnumNiveauListe::A2, 1),
            (EnumNiveauListe::B1, 2),
            (EnumNiveauListe::B2, 3),
            (EnumNiveauListe::C1, 4),
            (EnumNiveauListe::C2, 5),
        ];

        for (niveau, expected_id) in cases {
            assert_eq!(niveau.id(), expected_id);
        }
    }

    #[test]
    fn test_from_str_valid_values() {
        let cases = vec![
            ("A1", EnumNiveauListe::A1),
            ("A2", EnumNiveauListe::A2),
            ("B1", EnumNiveauListe::B1),
            ("B2", EnumNiveauListe::B2),
            ("C1", EnumNiveauListe::C1),
            ("C2", EnumNiveauListe::C2),
        ];

        for (input, expected) in cases {
            let parsed = EnumNiveauListe::from_str(input).expect("Debe parsear correctamente");
            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn test_from_str_invalid_value() {
        let invalid = "D1";
        let err = EnumNiveauListe::from_str(invalid).unwrap_err();

        assert_eq!(err.field, "NiveauListe");
        assert_eq!(err.message, format!("{invalid} is not a NiveauListe valid"));

        let opts = err.valid_options.expect("Debe incluir valid_options");
        let opts: Vec<String> = opts.into_iter().map(|d| d.to_owned()).collect();

        assert_eq!(
            opts,
            vec![
                "A1".to_string(),
                "A2".to_string(),
                "B1".to_string(),
                "B2".to_string(),
                "C1".to_string(),
                "C2".to_string(),
            ]
        );
    }

    #[test]
    fn test_as_str_matches_expected() {
        let cases = [
            (EnumNiveauListe::A1, "A1"),
            (EnumNiveauListe::A2, "A2"),
            (EnumNiveauListe::B1, "B1"),
            (EnumNiveauListe::B2, "B2"),
            (EnumNiveauListe::C1, "C1"),
            (EnumNiveauListe::C2, "C2"),
        ];

        for (niveau, expected) in cases {
            assert_eq!(niveau.as_str(), expected);
        }
    }

    #[test]
    fn try_from_i32_valid_values() {
        assert_eq!(EnumNiveauListe::try_from(0).unwrap(), EnumNiveauListe::A1);
        assert_eq!(EnumNiveauListe::try_from(1).unwrap(), EnumNiveauListe::A2);
        assert_eq!(EnumNiveauListe::try_from(2).unwrap(), EnumNiveauListe::B1);
        assert_eq!(EnumNiveauListe::try_from(3).unwrap(), EnumNiveauListe::B2);
        assert_eq!(EnumNiveauListe::try_from(4).unwrap(), EnumNiveauListe::C1);
        assert_eq!(EnumNiveauListe::try_from(5).unwrap(), EnumNiveauListe::C2);
    }

    #[test]
    fn try_from_i32_invalid_value_negative() {
        let err: InvalidValueError = EnumNiveauListe::try_from(-1).unwrap_err();

        assert_eq!(err.field, "NiveauListe");
        assert_eq!(err.message, "-1 is not a valid id for NiveauListe");

        assert_eq!(err.valid_options, None);
    }

    #[test]
    fn try_from_i32_invalid_value_out_of_range() {
        let err: InvalidValueError = EnumNiveauListe::try_from(6).unwrap_err();

        assert_eq!(err.field, "NiveauListe");
        assert_eq!(err.message, "6 is not a valid id for NiveauListe");

        assert_eq!(err.valid_options, None);
    }
}
