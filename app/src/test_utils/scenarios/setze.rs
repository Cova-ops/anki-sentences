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
            InputSetze {
                setze_spanisch: "Nos vemos mañana.".into(),
                setze_deutsch: "Wir sehen uns morgen.".into(),
                niveau: EnumNiveauListe::A1,
                thema: "daily_life".into(),
            },
            InputSetze {
                setze_spanisch: "¿Puedes ayudarme con esto?".into(),
                setze_deutsch: "Kannst du mir damit helfen?".into(),
                niveau: EnumNiveauListe::B2,
                thema: "communication".into(),
            },
        ],
        update: vec![],
        update_id: vec![],
    }
}
