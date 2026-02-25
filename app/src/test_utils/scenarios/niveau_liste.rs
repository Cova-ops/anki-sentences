use crate::{
    db::schemas::niveau_liste::{EnumNiveauListe, InputNiveauListe},
    test_utils::scenarios::Scenario,
};

pub fn scenario_niveau_liste() -> Scenario<InputNiveauListe> {
    Scenario {
        initial: EnumNiveauListe::ALL.iter().map(|r| r.to_new()).collect(),
        update: vec![],
        update_id: vec![],
    }
}
