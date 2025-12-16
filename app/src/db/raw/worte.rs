use color_eyre::eyre::Result;
use sql_model::FromRaw;

use crate::{
    db::schemas::{
        niveau_liste::NiveauListeSchema,
        worte::{RawWorteSchema as Raw, WorteSchema as Schema},
        worte_gender::WorteGenderSchema,
    },
    helpers::time::string_2_datetime,
};

impl FromRaw<Raw> for Schema {
    fn from_raw(r: Raw) -> Result<Self> {
        let gender_id = match r.gender_id {
            Some(v) => Some(WorteGenderSchema::from_id(v)?),
            None => None,
        };
        let niveau_id = NiveauListeSchema::from_id(r.niveau_id)?;

        let created_at = string_2_datetime(Some(r.created_at)).unwrap();
        let deleted_at = string_2_datetime(r.deleted_at);

        Ok(Schema {
            id: r.id,
            gram_type_id: vec![],
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

    fn from_vec_raw(data: Vec<Raw>) -> Result<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
}
