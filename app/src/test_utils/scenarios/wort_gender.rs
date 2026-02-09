use crate::test_utils::prelude::*;

pub fn scenario_wort_gender() -> Scenario<InputWortGender> {
    Scenario {
        initial: EnumWortGender::ALL.iter().map(|d| d.to_new()).collect(),
        update: vec![],
    }
}
