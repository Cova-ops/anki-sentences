use crate::db::{
    schemas::niveau_liste::EnumNiveauListe,
    traits::{SqlInsert, SqlUpdate},
};

#[derive(Debug, Clone)]
pub struct InputNiveauListe {
    pub niveau: EnumNiveauListe,
}

#[derive(Debug)]
pub struct SqlNiveauListe {
    pub id: i32,
    pub niveau: String,
}

impl From<InputNiveauListe> for SqlNiveauListe {
    fn from(value: InputNiveauListe) -> Self {
        Self {
            id: value.niveau.id(),
            niveau: value.niveau.as_str().to_string(),
        }
    }
}

impl SqlInsert for SqlNiveauListe {
    /// This orden:
    /// - id
    /// - niveau
    fn insert_params<'a>(&'a self) -> Vec<&'a dyn rusqlite::ToSql> {
        vec![&self.id, &self.niveau]
    }
}

impl SqlUpdate for SqlNiveauListe {}

#[cfg(test)]
mod tests_sql_niveau_liste {
    use super::*;

    mod from_input {
        use super::*;

        #[test]
        fn from_input_converts_a1_correctly() {
            let input = InputNiveauListe {
                niveau: EnumNiveauListe::A1,
            };

            let sql: SqlNiveauListe = input.into();

            assert_eq!(sql.id, 0);
            assert_eq!(sql.niveau, "A1".to_string());
        }

        #[test]
        fn from_input_converts_all_levels_correctly() {
            let cases = [
                (EnumNiveauListe::A1, 0, "A1"),
                (EnumNiveauListe::A2, 1, "A2"),
                (EnumNiveauListe::B1, 2, "B1"),
                (EnumNiveauListe::B2, 3, "B2"),
                (EnumNiveauListe::C1, 4, "C1"),
                (EnumNiveauListe::C2, 5, "C2"),
            ];

            for (niveau, id, s) in cases {
                let sql: SqlNiveauListe = InputNiveauListe { niveau }.into();
                assert_eq!(sql.id, id);
                assert_eq!(sql.niveau, s.to_string());
            }
        }
    }

    mod sql_params {
        use super::*;

        use rusqlite::{
            ToSql,
            types::{ToSqlOutput, Value, ValueRef},
        };

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
            let s = SqlNiveauListe {
                id: 1,
                niveau: String::from("A2"),
            };

            let params = s.insert_params();

            assert_eq!(to_value(params[0]), Value::Integer(1));
            assert_eq!(to_value(params[1]), Value::Text(String::from("A2")));
        }

        #[test]
        fn update_params() {
            let s = SqlNiveauListe {
                id: 1,
                niveau: String::from("A2"),
            };

            let params = s.update_params(&99);

            assert_eq!(to_value(params[0]), Value::Integer(1));
            assert_eq!(to_value(params[1]), Value::Text(String::from("A2")));
            assert_eq!(to_value(params[2]), Value::Integer(99));
        }
    }
}
