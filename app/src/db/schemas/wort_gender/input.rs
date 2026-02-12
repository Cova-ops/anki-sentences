use crate::db::{schemas::wort_gender::EnumWortGender, traits::SqlNew};

pub struct SqlWortGender {
    pub id: i32,
    pub gender: String,
    pub artikel: String,
}

#[derive(Debug, Clone)]
pub struct InputWortGender {
    pub gender: EnumWortGender,
}

impl From<InputWortGender> for SqlWortGender {
    fn from(value: InputWortGender) -> Self {
        Self {
            id: value.gender.id(),
            gender: value.gender.gender().to_string(),
            artikel: value.gender.artikel().to_string(),
        }
    }
}

impl SqlNew for SqlWortGender {
    type Params<'a>
        = (
        &'a dyn rusqlite::ToSql,
        &'a dyn rusqlite::ToSql,
        &'a dyn rusqlite::ToSql,
    )
    where
        Self: 'a;

    /// This orden:
    /// - id
    /// - gender
    /// - artikel
    fn to_params<'a>(&'a self) -> Self::Params<'a> {
        (&self.id, &self.gender, &self.artikel)
    }
}

#[cfg(test)]
mod tests_sql_wort_gender {
    use super::*;

    #[test]
    fn from_input_maps_enum_fields_correctly() {
        let cases = [
            (EnumWortGender::Maskuline, 0, "Maskuline", "der"),
            (EnumWortGender::Femenin, 1, "Femenin", "die"),
            (EnumWortGender::Neutrum, 2, "Neutrum", "das"),
            (EnumWortGender::Plural, 3, "Plural", "die"),
        ];

        for (gender_enum, id, gender, artikel) in cases {
            let input = InputWortGender {
                gender: gender_enum,
            };

            let sql: SqlWortGender = input.into();

            assert_eq!(sql.id, id);
            assert_eq!(sql.gender, gender);
            assert_eq!(sql.artikel, artikel);
        }
    }
}
