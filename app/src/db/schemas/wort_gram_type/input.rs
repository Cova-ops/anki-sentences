pub struct SqlWortGramType {
    pub id_worte: i32,
    pub id_gram_type: i32,
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests_sql_wort_gram_type {
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
