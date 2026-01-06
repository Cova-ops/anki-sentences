use crate::{db::schemas::worte_gender::NewWorteGenderSchema, test_utils::scenarios::Scenario};

pub fn scenario_worte_gender() -> Scenario<NewWorteGenderSchema> {
    Scenario {
        initial: vec![
            NewWorteGenderSchema::new(0, "Maskuline", "der"),
            NewWorteGenderSchema::new(1, "Femenin", "die"),
            NewWorteGenderSchema::new(2, "Neutrum", "das"),
            NewWorteGenderSchema::new(3, "Plural", "die"),
        ],
        update: vec![
            NewWorteGenderSchema::new(0, "Maskuline test", "der test"),
            NewWorteGenderSchema::new(1, "Femenin test", "die test"),
            NewWorteGenderSchema::new(2, "Neutrum test", "das test"),
        ],
    }
}
