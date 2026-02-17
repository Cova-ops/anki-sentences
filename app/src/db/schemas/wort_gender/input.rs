use crate::db::{
    schemas::wort_gender::EnumWortGender,
    traits::{SqlInsert, SqlUpdate},
};

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

impl SqlInsert for SqlWortGender {
    /// This orden:
    /// - id
    /// - gender
    /// - artikel
    fn insert_params<'a>(&'a self) -> Vec<&'a dyn rusqlite::ToSql> {
        vec![&self.id, &self.gender, &self.artikel]
    }
}

impl SqlUpdate for SqlWortGender {}

#[cfg(test)]
mod tests_sql_wort_gender {
    use super::*;

    mod from_input {
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

    mod sql_params {
        use rusqlite::{
            ToSql,
            types::{ToSqlOutput, Value, ValueRef},
        };

        use super::*;

        fn to_value(p: &dyn ToSql) -> Value {
            match p.to_sql().expect("to_sql should work") {
                ToSqlOutput::Owned(v) => v,
                ToSqlOutput::Borrowed(vr) => match vr {
                    ValueRef::Null => Value::Null,
                    ValueRef::Integer(i) => Value::Integer(i),
                    ValueRef::Real(f) => Value::Real(f),
                    ValueRef::Text(t) => Value::Text(String::from_utf8_lossy(t).into_owned()),
                    ValueRef::Blob(b) => Value::Blob(b.to_vec()),
                },
                _ => panic!(""),
            }
        }

        #[test]
        fn insert_params() {
            let s = SqlWortGender {
                id: 0,
                gender: String::from("Maskuline"),
                artikel: String::from("der"),
            };

            let params = s.insert_params();

            assert_eq!(to_value(params[0]), Value::Integer(0));
            assert_eq!(to_value(params[1]), Value::Text("Maskuline".to_string()));
            assert_eq!(to_value(params[2]), Value::Text("der".to_string()));
        }

        #[test]
        fn update_params() {
            let s = SqlWortGender {
                id: 0,
                gender: String::from("Maskuline"),
                artikel: String::from("der"),
            };

            let params = s.update_params(&99);

            assert_eq!(to_value(params[0]), Value::Integer(0));
            assert_eq!(to_value(params[1]), Value::Text("Maskuline".to_string()));
            assert_eq!(to_value(params[2]), Value::Text("der".to_string()));
            assert_eq!(to_value(params[3]), Value::Integer(99));
        }
    }
}
