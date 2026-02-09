use crate::test_utils::prelude::*;

pub fn scenario_wort_gram_type() -> Scenario<InputWortGramType> {
    Scenario {
        initial: vec![
            InputWortGramType {
                id_worte: 1,
                id_gram_type: 1,
            },
            InputWortGramType {
                id_worte: 1,
                id_gram_type: 2,
            },
            InputWortGramType {
                id_worte: 2,
                id_gram_type: 2,
            },
        ],
        update: vec![],
    }
}
