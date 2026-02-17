use crate::db::{
    schemas::gram_type::EnumGramType,
    traits::{SqlInsert, SqlUpdate},
};

#[derive(Debug, Clone)]
pub struct InputGramType {
    pub gram: EnumGramType,
}

#[derive(Debug, Clone)]
pub struct SqlGramType {
    pub id: i32,
    pub code: String,
    pub name: String,
}

impl From<InputGramType> for SqlGramType {
    fn from(value: InputGramType) -> Self {
        Self {
            id: value.gram.id(),
            code: value.gram.to_code().to_string(),
            name: value.gram.to_name().to_string(),
        }
    }
}

impl SqlInsert for SqlGramType {
    /// This orden:
    /// - id
    /// - code
    /// - name
    fn insert_params<'a>(&'a self) -> Vec<&'a dyn rusqlite::ToSql> {
        vec![&self.id, &self.code, &self.name]
    }
}

impl SqlUpdate for SqlGramType {}

#[cfg(test)]
mod tests_sql_gram_type {
    use super::*;

    mod from_input {
        use super::*;

        #[test]
        fn from_input_gram_type_noun_common() {
            let input = InputGramType {
                gram: EnumGramType::NounCommon,
            };

            let sql: SqlGramType = input.into();

            assert_eq!(sql.id, 0);
            assert_eq!(sql.code, "noun_common");
            assert_eq!(sql.name, "Sustantivo común");
        }

        #[test]
        fn from_input_gram_type_verb_main() {
            let input = InputGramType {
                gram: EnumGramType::VerbMain,
            };

            let sql: SqlGramType = input.into();

            assert_eq!(sql.id, 2);
            assert_eq!(sql.code, "verb_main");
            assert_eq!(sql.name, "Verbo léxico");
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
            let s = SqlGramType {
                id: 1,
                code: String::from("verb_main"),
                name: String::from("Verb Main"),
            };

            let params = s.insert_params();

            assert_eq!(to_value(params[0]), Value::Integer(1));
            assert_eq!(to_value(params[1]), Value::Text(String::from("verb_main")));
            assert_eq!(to_value(params[2]), Value::Text(String::from("Verb Main")));
        }

        #[test]
        fn update_params() {
            let s = SqlGramType {
                id: 1,
                code: String::from("verb_main"),
                name: String::from("Verb Main"),
            };

            let params = s.update_params(&10);

            assert_eq!(to_value(params[0]), Value::Integer(1));
            assert_eq!(to_value(params[1]), Value::Text(String::from("verb_main")));
            assert_eq!(to_value(params[2]), Value::Text(String::from("Verb Main")));
            assert_eq!(to_value(params[3]), Value::Integer(10));
        }
    }
}
