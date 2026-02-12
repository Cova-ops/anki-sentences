use crate::{
    db::schemas::wort_gender::{EnumWortGender, InputWortGender},
    test_utils::scenarios::Scenario,
};

pub fn scenario_wort_gender() -> Scenario<InputWortGender> {
    Scenario {
        initial: EnumWortGender::ALL.iter().map(|d| d.to_new()).collect(),
        update: vec![],
    }
}
