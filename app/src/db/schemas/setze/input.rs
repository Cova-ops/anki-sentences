use crate::db::{
    schemas::niveau_liste::EnumNiveauListe,
    traits::{SqlInsert, SqlUpdate},
};

#[derive(Debug)]
pub struct SqlSetze {
    pub setze_spanisch: String,
    pub setze_deutsch: String,
    pub niveau_id: i32,
    pub thema: String,
}

#[derive(Debug, Clone)]
pub struct InputSetze {
    pub setze_spanisch: String,
    pub setze_deutsch: String,
    pub niveau: EnumNiveauListe,
    pub thema: String,
}

impl From<InputSetze> for SqlSetze {
    fn from(value: InputSetze) -> Self {
        Self {
            setze_spanisch: value.setze_spanisch,
            setze_deutsch: value.setze_deutsch,
            niveau_id: value.niveau.id(),
            thema: value.thema,
        }
    }
}

impl SqlInsert for SqlSetze {
    /// This orden:
    /// - setze_spanisch
    /// - setze_deutsch
    /// - niveau_id
    /// - thema
    fn insert_params<'a>(&'a self) -> Vec<&'a dyn rusqlite::ToSql> {
        vec![
            &self.setze_spanisch,
            &self.setze_deutsch,
            &self.niveau_id,
            &self.thema,
        ]
    }
}

impl SqlUpdate for SqlSetze {}

#[cfg(test)]
mod tests_sql_setze_from_input {
    use crate::db::schemas::{
        niveau_liste::EnumNiveauListe,
        setze::{InputSetze, SqlSetze},
    };
    use rusqlite::ToSql;
    use rusqlite::types::{ToSqlOutput, Value, ValueRef};

    mod from_input_setze {
        use super::*;

        #[test]
        fn input_setze_into_sql_setze_maps_fields_correctly() {
            let input = InputSetze {
                setze_spanisch: "Estoy aprendiendo alemán.".to_string(),
                setze_deutsch: "Ich lerne Deutsch.".to_string(),
                niveau: EnumNiveauListe::A2,
                thema: "lernen".to_string(),
            };

            let sql: SqlSetze = input.into();

            assert_eq!(sql.setze_spanisch, "Estoy aprendiendo alemán.");
            assert_eq!(sql.setze_deutsch, "Ich lerne Deutsch.");
            assert_eq!(sql.niveau_id, EnumNiveauListe::A2.id());
            assert_eq!(sql.thema, "lernen");
        }
    }

    mod sql_new {
        use crate::db::traits::{SqlInsert, SqlUpdate};

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
            let s = SqlSetze {
                setze_spanisch: "Yo corro diario".to_string(),
                setze_deutsch: "Ich laufe jeden Tag".to_string(),
                niveau_id: 2,
                thema: "Sport".to_string(),
            };

            let params = s.insert_params();

            assert_eq!(
                to_value(params[0]),
                Value::Text("Yo corro diario".to_string())
            );
            assert_eq!(
                to_value(params[1]),
                Value::Text("Ich laufe jeden Tag".to_string())
            );
            assert_eq!(to_value(params[2]), Value::Integer(2));
            assert_eq!(to_value(params[3]), Value::Text("Sport".to_string()));
        }

        #[test]
        fn update_params() {
            let s = SqlSetze {
                setze_spanisch: "Yo corro diario".to_string(),
                setze_deutsch: "Ich laufe jeden Tag".to_string(),
                niveau_id: 2,
                thema: "Sport".to_string(),
            };

            let params = s.update_params(&99);

            assert_eq!(
                to_value(params[0]),
                Value::Text("Yo corro diario".to_string())
            );
            assert_eq!(
                to_value(params[1]),
                Value::Text("Ich laufe jeden Tag".to_string())
            );
            assert_eq!(to_value(params[2]), Value::Integer(2));
            assert_eq!(to_value(params[3]), Value::Text("Sport".to_string()));
            assert_eq!(to_value(params[4]), Value::Integer(99));
        }
    }
}
