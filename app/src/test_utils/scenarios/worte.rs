use crate::{db::schemas::worte::NewWorteSchema as New, test_utils::scenarios::Scenario};

pub fn scenario_worte_schema() -> Scenario<New> {
    Scenario {
        initial: vec![
            New {
                gram_type: vec![1],
                gender_id: Some(1),
                worte_de: "Hund".into(),
                worte_es: "Perro".into(),
                plural: Some("Hunde".into()),
                niveau_id: 1,
                example_de: "Beispiel".into(),
                example_es: "Ejemplo".into(),
                verb_aux: None,
                trennbar: None,
                reflexiv: None,
            },
            New {
                gram_type: vec![2, 3],
                gender_id: None,
                worte_de: "laufen".into(),
                worte_es: "correr".into(),
                plural: None,
                niveau_id: 2,
                example_de: "Beispiel".into(),
                example_es: "Ejemplo".into(),
                verb_aux: Some("sein".into()),
                trennbar: Some(false),
                reflexiv: Some(false),
            },
        ],
        update: vec![
            New {
                gram_type: vec![1],
                gender_id: Some(1),
                worte_de: "Hund".into(),
                worte_es: "Perro".into(),
                plural: Some("Hunde".into()),
                niveau_id: 1,
                example_de: "Beispiel".into(),
                example_es: "Ejemplo".into(),
                verb_aux: None,
                trennbar: None,
                reflexiv: None,
            },
            New {
                gram_type: vec![2, 3],
                gender_id: None,
                worte_de: "laufen".into(),
                worte_es: "correr".into(),
                plural: None,
                niveau_id: 2,
                example_de: "Beispiel".into(),
                example_es: "Ejemplo".into(),
                verb_aux: Some("sein".into()),
                trennbar: Some(false),
                reflexiv: Some(false),
            },
        ],
    }
}
