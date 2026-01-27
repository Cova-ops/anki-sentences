use crate::{
    db::schemas::worte::{NewWorteSchema, WorteSchema},
    test_utils::{
        snapshots::{
            gram_type::GramTypeSnapshot, niveau_liste::NiveauListeSnapshot,
            worte_gender::WorteGenderSnapshot,
        },
        traits::{AssertEqFields, SnapshotFields},
    },
};

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorteSnapshot {
    pub id: i32,
    pub gram_type_id: Vec<GramTypeSnapshot>,
    pub gender_id: Option<WorteGenderSnapshot>,

    pub worte_de: String,
    pub worte_es: String,
    pub plural: Option<String>,

    pub niveau_id: NiveauListeSnapshot,
    pub example_de: String,
    pub example_es: String,

    // verbos
    pub verb_aux: Option<String>,
    pub trennbar: Option<bool>,
    pub reflexiv: Option<bool>,

    // placeholders
    pub created_at: String,
    pub deleted_at: String,
}

impl SnapshotFields for WorteSchema {
    type Output = WorteSnapshot;

    fn snapshot(self) -> WorteSnapshot {
        WorteSnapshot {
            id: self.id,
            gender_id: self.gender_id.snapshot(),
            gram_type_id: self.gram_type_id.snapshot(),

            worte_de: self.worte_de,
            worte_es: self.worte_es,
            plural: self.plural,

            niveau_id: self.niveau_id.snapshot(),
            example_de: self.example_de,
            example_es: self.example_es,

            verb_aux: self.verb_aux,
            trennbar: self.trennbar,
            reflexiv: self.reflexiv,

            created_at: "<created_at>".into(),
            deleted_at: "<deleted_at>".into(),
        }
    }

    fn snapshot_ref(&self) -> WorteSnapshot {
        WorteSnapshot {
            id: self.id,
            gender_id: self.gender_id.snapshot_ref(),
            gram_type_id: self.gram_type_id.snapshot_ref(),

            worte_de: self.worte_de.clone(),
            worte_es: self.worte_es.clone(),
            plural: self.plural.clone(),

            niveau_id: self.niveau_id.snapshot_ref(),
            example_de: self.example_de.clone(),
            example_es: self.example_es.clone(),

            verb_aux: self.verb_aux.clone(),
            trennbar: self.trennbar,
            reflexiv: self.reflexiv,

            created_at: "<created_at>".into(),
            deleted_at: "<deleted_at>".into(),
        }
    }
}

impl SnapshotFields for Vec<WorteSchema> {
    type Output = Vec<WorteSnapshot>;

    fn snapshot(self) -> Vec<WorteSnapshot> {
        self.into_iter().map(|w| w.snapshot()).collect()
    }
    fn snapshot_ref(&self) -> Self::Output {
        self.iter().map(|w| w.snapshot_ref()).collect()
    }
}

// Schema vs New (1 a 1)
impl AssertEqFields<NewWorteSchema> for WorteSchema {
    fn assert_eq_fields(&self, expected: &NewWorteSchema) {
        assert_eq!(
            self.gram_type_id.iter().map(|w| w.id).collect::<Vec<i32>>(),
            expected.gram_type
        );
        assert_eq!(self.gender_id.as_ref().map(|w| w.id), expected.gender_id);
        assert_eq!(self.worte_de, expected.worte_de);
        assert_eq!(self.worte_es, expected.worte_es);
        assert_eq!(self.plural, expected.plural);
        assert_eq!(self.niveau_id.id, expected.niveau_id);
        assert_eq!(self.example_de, expected.example_de);
        assert_eq!(self.example_es, expected.example_es);
        assert_eq!(self.verb_aux, expected.verb_aux);
        assert_eq!(self.trennbar, expected.trennbar);
        assert_eq!(self.reflexiv, expected.reflexiv);
    }
}

// Vec<Schema> vs Vec<New> (zip)
impl AssertEqFields<Vec<NewWorteSchema>> for Vec<WorteSchema> {
    fn assert_eq_fields(&self, expected: &Vec<NewWorteSchema>) {
        assert_eq!(self.len(), expected.len(), "Length mismatch");

        for (a, e) in self.iter().zip(expected.iter()) {
            a.assert_eq_fields(e);
        }
    }
}
