use crate::{
    db::schemas::{niveau_liste::EnumNiveauListe, setze::InputSetze},
    test_utils::scenarios::Scenario,
};

pub fn scenario_setze() -> Scenario<InputSetze> {
    Scenario {
        initial: vec![
            InputSetze {
                setze_spanisch: "Estoy aprendiendo alemán.".into(),
                setze_deutsch: "Ich lerne Deutsch.".into(),
                niveau: EnumNiveauListe::A2,
                thema: "learning".into(),
            },
            InputSetze {
                setze_spanisch: "Ella trabaja aquí.".into(),
                setze_deutsch: "Sie arbeitet hier.".into(),
                niveau: EnumNiveauListe::B1,
                thema: "work".into(),
            },
        ],
        update: vec![],
        update_id: vec![],
    }
}
