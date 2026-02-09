use crate::test_utils::prelude::*;

pub fn scenario_niveau_liste() -> Scenario<InputNiveauListe> {
    Scenario {
        initial: EnumNiveauListe::ALL.iter().map(|r| r.to_new()).collect(),
        update: vec![],
    }
}
