use crate::db::schemas::gram_type::EnumGramType;

#[derive(Debug, Clone)]
pub struct SqlGramType {
    pub id: i32,
    pub code: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct InputGramType {
    pub gram: EnumGramType,
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

#[cfg(test)]
mod tests_sql_gram_type {
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
