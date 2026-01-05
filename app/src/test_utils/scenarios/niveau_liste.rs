use crate::{db::schemas::niveau_liste::NewNiveauListeSchema, test_utils::scenarios::Scenario};

pub fn scenario_niveau_liste_schema() -> Scenario<NewNiveauListeSchema> {
    Scenario {
        initial: vec![
            NewNiveauListeSchema::new(0, "A1"),
            NewNiveauListeSchema::new(1, "A2"),
            NewNiveauListeSchema::new(2, "B1"),
            NewNiveauListeSchema::new(3, "B2"),
            NewNiveauListeSchema::new(4, "C1"),
            NewNiveauListeSchema::new(5, "C2"),
        ],
        update: vec![
            NewNiveauListeSchema::new(0, "A1 test"),
            NewNiveauListeSchema::new(1, "A2 test"),
            NewNiveauListeSchema::new(2, "B1 test"),
        ],
    }
}
