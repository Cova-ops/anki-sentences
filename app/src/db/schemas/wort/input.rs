use crate::db::{
    schemas::{
        gram_type::EnumGramType, niveau_liste::EnumNiveauListe, wort_audio::SqlWortAudio,
        wort_gender::EnumWortGender,
    },
    traits::{SqlInsert, SqlUpdate},
};

#[derive(Debug)]
pub struct SqlWort {
    pub gram_type_ids: Vec<i32>,
    pub gender_id: Option<i32>,
    pub worte_de: String,
    pub worte_es: String,
    pub plural: Option<String>,
    pub niveau_id: i32,
    pub example_de: String,
    pub example_es: String,

    // nur verben
    pub verb_aux: Option<String>,
    pub trennbar: Option<bool>,
    pub reflexiv: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct InputWort {
    pub gram_type: Vec<EnumGramType>,
    pub gender: Option<EnumWortGender>,
    pub worte_de: String,
    pub worte_es: String,
    pub plural: Option<String>,
    pub niveau: EnumNiveauListe,
    pub example_de: String,
    pub example_es: String,

    // nur verben
    pub verb_aux: Option<String>,
    pub trennbar: Option<bool>,
    pub reflexiv: Option<bool>,
}

impl From<InputWort> for SqlWort {
    fn from(value: InputWort) -> Self {
        Self {
            gram_type_ids: value.gram_type.iter().map(|d| d.id()).collect(),
            gender_id: value.gender.as_ref().map(|d| d.id()),
            worte_de: value.worte_de,
            worte_es: value.worte_es,
            plural: value.plural,
            niveau_id: value.niveau.id(),
            example_de: value.example_de,
            example_es: value.example_es,

            // nur verben
            verb_aux: value.verb_aux,
            trennbar: value.trennbar,
            reflexiv: value.reflexiv,
        }
    }
}

impl SqlInsert for SqlWort {
    /// This orden:
    /// - gender_id
    /// - worte_de
    /// - worte_es
    /// - plural
    /// - niveau_id
    /// - example_de
    /// - example_es
    /// - verb_aux
    /// - trennbar
    /// - reflexiv
    fn insert_params<'a>(&'a self) -> Vec<&'a dyn rusqlite::ToSql> {
        vec![
            &self.gender_id,
            &self.worte_de,
            &self.worte_es,
            &self.plural,
            &self.niveau_id,
            &self.example_de,
            &self.example_es,
            &self.verb_aux,
            &self.trennbar,
            &self.reflexiv,
        ]
    }
}

impl SqlUpdate for SqlWort {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::schemas::{
        gram_type::EnumGramType, niveau_liste::EnumNiveauListe, wort_gender::EnumWortGender,
    };

    mod from_input {
        use super::*;

        #[test]
        fn input_wort_to_sql_wort_maps_ids_and_fields() {
            let input = InputWort {
                gram_type: vec![EnumGramType::NounCommon, EnumGramType::VerbMain],
                gender: Some(EnumWortGender::Maskuline),
                worte_de: "Hund".to_string(),
                worte_es: "Perro".to_string(),
                plural: Some("Hunde".to_string()),
                niveau: EnumNiveauListe::A2,
                example_de: "Der Hund bellt.".to_string(),
                example_es: "El perro ladra.".to_string(),
                verb_aux: Some("haben".to_string()),
                trennbar: Some(false),
                reflexiv: Some(false),
            };

            let sql: SqlWort = input.into();

            // ids
            assert_eq!(
                sql.gram_type_ids,
                vec![EnumGramType::NounCommon.id(), EnumGramType::VerbMain.id()]
            );
            assert_eq!(sql.gender_id, Some(EnumWortGender::Maskuline.id()));
            assert_eq!(sql.niveau_id, EnumNiveauListe::A2.id());

            // passthrough fields
            assert_eq!(sql.worte_de, "Hund");
            assert_eq!(sql.worte_es, "Perro");
            assert_eq!(sql.plural, Some("Hunde".to_string()));
            assert_eq!(sql.example_de, "Der Hund bellt.");
            assert_eq!(sql.example_es, "El perro ladra.");
            assert_eq!(sql.verb_aux, Some("haben".to_string()));
            assert_eq!(sql.trennbar, Some(false));
            assert_eq!(sql.reflexiv, Some(false));
        }

        #[test]
        fn input_wort_to_sql_wort_none_gender() {
            let input = InputWort {
                gram_type: vec![EnumGramType::Adjective],
                gender: None,
                worte_de: "müde".to_string(),
                worte_es: "cansado".to_string(),
                plural: None,
                niveau: EnumNiveauListe::A1,
                example_de: "Ich bin müde.".to_string(),
                example_es: "Estoy cansado.".to_string(),
                verb_aux: None,
                trennbar: None,
                reflexiv: None,
            };

            let sql: SqlWort = input.into();

            assert_eq!(sql.gram_type_ids, vec![EnumGramType::Adjective.id()]);
            assert_eq!(sql.gender_id, None);
            assert_eq!(sql.niveau_id, EnumNiveauListe::A1.id());

            assert_eq!(sql.plural, None);
            assert_eq!(sql.verb_aux, None);
            assert_eq!(sql.trennbar, None);
            assert_eq!(sql.reflexiv, None);
        }

        #[test]
        fn input_wort_to_sql_wort_preserves_gram_type_order() {
            let input = InputWort {
                gram_type: vec![
                    EnumGramType::VerbSeparable,
                    EnumGramType::VerbMain,
                    EnumGramType::PrefixSeparable,
                ],
                gender: None,
                worte_de: "weitergehen".to_string(),
                worte_es: "seguir".to_string(),
                plural: None,
                niveau: EnumNiveauListe::B1,
                example_de: "Wir gehen weiter.".to_string(),
                example_es: "Seguimos.".to_string(),
                verb_aux: Some("sein".to_string()),
                trennbar: Some(true),
                reflexiv: Some(false),
            };

            let sql: SqlWort = input.into();

            assert_eq!(
                sql.gram_type_ids,
                vec![
                    EnumGramType::VerbSeparable.id(),
                    EnumGramType::VerbMain.id(),
                    EnumGramType::PrefixSeparable.id(),
                ]
            );
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
            let s = SqlWort {
                gram_type_ids: vec![1, 2, 3],
                gender_id: Some(1),
                worte_de: String::from("Wort auf Deutsch"),
                worte_es: String::from("Palabra en aleman"),
                plural: None,
                niveau_id: 1,
                example_de: String::from("Beispiel auf Deutsch"),
                example_es: String::from("Ejemplo en aleman"),
                verb_aux: None,
                trennbar: None,
                reflexiv: None,
            };

            // insert_params ignores gram_types_ids
            let params = s.insert_params();

            assert_eq!(to_value(params[0]), Value::Integer(1));
            assert_eq!(
                to_value(params[1]),
                Value::Text(String::from("Wort auf Deutsch"))
            );
            assert_eq!(
                to_value(params[2]),
                Value::Text(String::from("Palabra en aleman"))
            );
            assert_eq!(to_value(params[3]), Value::Null);
            assert_eq!(to_value(params[4]), Value::Integer(1));
            assert_eq!(
                to_value(params[5]),
                Value::Text(String::from("Beispiel auf Deutsch"))
            );
            assert_eq!(
                to_value(params[6]),
                Value::Text(String::from("Ejemplo en aleman"))
            );
            assert_eq!(to_value(params[7]), Value::Null);
            assert_eq!(to_value(params[8]), Value::Null);
            assert_eq!(to_value(params[9]), Value::Null);
        }

        #[test]
        fn update_params() {
            let s = SqlWort {
                gram_type_ids: vec![1, 2, 3],
                gender_id: Some(1),
                worte_de: String::from("Wort auf Deutsch"),
                worte_es: String::from("Palabra en aleman"),
                plural: None,
                niveau_id: 1,
                example_de: String::from("Beispiel auf Deutsch"),
                example_es: String::from("Ejemplo en aleman"),
                verb_aux: None,
                trennbar: None,
                reflexiv: None,
            };

            // update_params ignores gram_types_ids
            let params = s.update_params(&99);

            assert_eq!(to_value(params[0]), Value::Integer(1));
            assert_eq!(
                to_value(params[1]),
                Value::Text(String::from("Wort auf Deutsch"))
            );
            assert_eq!(
                to_value(params[2]),
                Value::Text(String::from("Palabra en aleman"))
            );
            assert_eq!(to_value(params[3]), Value::Null);
            assert_eq!(to_value(params[4]), Value::Integer(1));
            assert_eq!(
                to_value(params[5]),
                Value::Text(String::from("Beispiel auf Deutsch"))
            );
            assert_eq!(
                to_value(params[6]),
                Value::Text(String::from("Ejemplo en aleman"))
            );
            assert_eq!(to_value(params[7]), Value::Null);
            assert_eq!(to_value(params[8]), Value::Null);
            assert_eq!(to_value(params[9]), Value::Null);
            assert_eq!(to_value(params[10]), Value::Integer(99));
        }
    }
}
