use crate::db::traits::{SqlInsert, SqlUpdate};

#[derive(Debug, Clone)]
pub struct SqlWortGramType {
    pub id_worte: i32,
    pub id_gram_type: i32,
}

#[derive(Debug, Clone)]
pub struct InputWortGramType {
    pub id_worte: i32,
    pub id_gram_type: i32,
}

impl From<InputWortGramType> for SqlWortGramType {
    fn from(value: InputWortGramType) -> Self {
        Self {
            id_worte: value.id_worte,
            id_gram_type: value.id_gram_type,
        }
    }
}

impl SqlInsert for SqlWortGramType {
    /// This orden:
    /// - id_worte
    /// - id_gram_type
    fn insert_params<'a>(&'a self) -> Vec<&'a dyn rusqlite::ToSql> {
        vec![&self.id_worte, &self.id_gram_type]
    }
}

impl SqlUpdate for SqlWortGramType {}

#[cfg(test)]
mod tests_sql_wort_gram_type {
    use super::*;

    mod from_input {
        use super::*;

        #[test]
        fn from_input_copies_all_fields() {
            let input = InputWortGramType {
                id_worte: 42,
                id_gram_type: 7,
            };

            let sql: SqlWortGramType = input.into();

            assert_eq!(sql.id_worte, 42);
            assert_eq!(sql.id_gram_type, 7);
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
            let s = SqlWortGramType {
                id_worte: 1,
                id_gram_type: 2,
            };

            let params = s.insert_params();

            assert_eq!(to_value(params[0]), Value::Integer(1));
            assert_eq!(to_value(params[1]), Value::Integer(2));
        }

        #[test]
        fn update_params() {
            let s = SqlWortGramType {
                id_worte: 1,
                id_gram_type: 2,
            };

            let params = s.update_params(&99);

            assert_eq!(to_value(params[0]), Value::Integer(1));
            assert_eq!(to_value(params[1]), Value::Integer(2));
            assert_eq!(to_value(params[2]), Value::Integer(99));
        }
    }
}
