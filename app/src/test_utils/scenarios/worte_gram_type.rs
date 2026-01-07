use crate::{
    db::schemas::worte_gram_type::NewWorteGramTypeSchema, test_utils::scenarios::Scenario,
};

pub fn scenario_worte_gram_type() -> Scenario<NewWorteGramTypeSchema> {
    Scenario {
        initial: vec![
            NewWorteGramTypeSchema {
                id_worte: 1,
                id_gram_type: 1,
            },
            NewWorteGramTypeSchema {
                id_worte: 1,
                id_gram_type: 2,
            },
            NewWorteGramTypeSchema {
                id_worte: 2,
                id_gram_type: 2,
            },
        ],
        update: vec![],
    }
}
