use color_eyre::eyre::Result;

use crate::{
    db::{
        schemas::{
            gender_worte::GenderWorteSchema,
            niveau_worte::NiveauWorteSchema,
            worte::{RawWorteSchema, WorteSchema},
        },
        traits::FromRaw,
    },
    helpers::time::string_2_datetime,
};

impl FromRaw<RawWorteSchema> for WorteSchema {
    fn from_raw(r: RawWorteSchema) -> Result<Self> {
        let gender_id: Option<GenderWorteSchema> = match r.gender_id {
            Some(v) => Some(GenderWorteSchema::from_id(v)?),
            None => None,
        };
        let niveau_id: NiveauWorteSchema = NiveauWorteSchema::from_id(r.niveau_id)?;

        let created_at = string_2_datetime(Some(r.created_at)).unwrap();
        let deleted_at = string_2_datetime(r.deleted_at);

        Ok(WorteSchema {
            id: r.id,
            gender_id,
            worte_de: r.worte_de,
            worte_es: r.worte_es,
            plural: r.plural,
            niveau_id,
            example_de: r.example_de,
            example_es: r.example_es,
            verb_aux: r.verb_aux,
            trennbar: r.trennbar,
            reflexiv: r.reflexiv,
            created_at,
            deleted_at,
        })
    }

    fn from_vec_raw(data: Vec<RawWorteSchema>) -> Result<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
}
