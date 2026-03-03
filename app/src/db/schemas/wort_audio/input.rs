use crate::db::traits::{SqlInsert, SqlUpdate};

#[derive(Debug, Clone)]
pub struct SqlWortAudio {
    pub wort_id: i32,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InputWortAudio {
    pub wort_id: i32,
    pub audio_name_es: Option<String>,
    pub audio_name_de: Option<String>,
}

impl From<InputWortAudio> for SqlWortAudio {
    fn from(value: InputWortAudio) -> Self {
        Self {
            wort_id: value.wort_id,
            audio_name_es: value.audio_name_es,
            audio_name_de: value.audio_name_de,
        }
    }
}

impl SqlInsert for SqlWortAudio {
    /// This orden:
    /// - wort_id
    /// - audio_name_es
    /// - audio_name_de
    fn insert_params<'a>(&'a self) -> Vec<&'a dyn rusqlite::ToSql> {
        vec![&self.wort_id, &self.audio_name_es, &self.audio_name_de]
    }
}

impl SqlUpdate for SqlWortAudio {}

#[cfg(test)]
mod tests {
    use super::*;

    mod from_input {
        use super::*;

        #[test]
        fn all_some() {
            let input = InputWortAudio {
                wort_id: 42,
                audio_name_es: Some("hola_es.mp3".to_string()),
                audio_name_de: Some("hallo_de.mp3".to_string()),
            };

            let sql = SqlWortAudio::from(input);

            assert_eq!(sql.wort_id, 42);
            assert_eq!(sql.audio_name_es, Some("hola_es.mp3".to_string()));
            assert_eq!(sql.audio_name_de, Some("hallo_de.mp3".to_string()));
        }

        #[test]
        fn one_none() {
            let input = InputWortAudio {
                wort_id: 7,
                audio_name_es: Some("solo_es.mp3".to_string()),
                audio_name_de: None,
            };

            let sql = SqlWortAudio::from(input);

            assert_eq!(sql.wort_id, 7);
            assert_eq!(sql.audio_name_es, Some("solo_es.mp3".to_string()));
            assert_eq!(sql.audio_name_de, None);
        }

        #[test]
        fn all_none() {
            let input = InputWortAudio {
                wort_id: 99,
                audio_name_es: None,
                audio_name_de: None,
            };

            let sql = SqlWortAudio::from(input);

            assert_eq!(sql.wort_id, 99);
            assert_eq!(sql.audio_name_es, None);
            assert_eq!(sql.audio_name_de, None);
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
            let s = SqlWortAudio {
                wort_id: 1,
                audio_name_es: Some(String::from("audio_es")),
                audio_name_de: Some(String::from("audio_de")),
            };

            let params = s.insert_params();

            assert_eq!(to_value(params[0]), Value::Integer(1));
            assert_eq!(to_value(params[1]), Value::Text("audio_es".to_string()));
            assert_eq!(to_value(params[2]), Value::Text("audio_de".to_string()));
        }

        #[test]
        fn update_params() {
            let s = SqlWortAudio {
                wort_id: 1,
                audio_name_es: Some(String::from("audio_es")),
                audio_name_de: Some(String::from("audio_de")),
            };

            let params = s.update_params(&99);

            assert_eq!(to_value(params[0]), Value::Integer(1));
            assert_eq!(to_value(params[1]), Value::Text("audio_es".to_string()));
            assert_eq!(to_value(params[2]), Value::Text("audio_de".to_string()));
            assert_eq!(to_value(params[3]), Value::Integer(99));
        }
    }
}
