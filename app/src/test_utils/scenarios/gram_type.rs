use crate::test_utils::prelude::*;

pub fn scenario_gram_type() -> Scenario<InputGramType> {
    Scenario {
        initial: EnumGramType::ALL.iter().map(|d| d.to_new()).collect(),
        update: vec![],
    }
}
