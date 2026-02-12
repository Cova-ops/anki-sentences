use crate::{db::schemas::gram_type::InputGramType, test_utils::scenarios::Scenario};

pub fn scenario_gram_type() -> Scenario<InputGramType> {
    Scenario {
        initial: EnumGramType::ALL.iter().map(|d| d.to_new()).collect(),
        update: vec![],
    }
}
