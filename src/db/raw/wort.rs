use color_eyre::eyre::Result;

use crate::{
    db::{
        schemas::{
            gender_worte::GenderWorteSchema,
            niveau_worte::NiveauWorteSchema,
            wort::{NewWortSchema, RawWortSchema, WortSchema},
        },
        traits::FromRaw,
    },
    helpers::time::string_2_datetime,
};

impl NewWortSchema {
    pub fn new(
        id: i32,
        gender_id: Option<i32>,
        worte_de: String,
        worte_es: String,
        plural: Option<String>,
        niveau_id: Option<i32>,
        example_de: Option<String>,
        example_es: Option<String>,

        verb_aux: Option<String>,
        trennbar: Option<bool>,
        reflexiv: Option<bool>,
    ) -> Self {
        Self {
            id,
            gender_id,
            worte_de,
            worte_es,
            plural,
            niveau_id,
            example_de,
            example_es,
            verb_aux,
            trennbar,
            reflexiv,
        }
    }
}

impl FromRaw<RawWortSchema> for WortSchema {
    fn from_raw(r: RawWortSchema) -> Result<Self> {
        let gender_id: Option<GenderWorteSchema> = match r.gender_id {
            Some(v) => Some(GenderWorteSchema::from_id(v)?),
            None => None,
        };
        let niveau_id: Option<NiveauWorteSchema> = match r.niveau_id {
            Some(v) => Some(NiveauWorteSchema::from_id(v)?),
            None => None,
        };

        let created_at = string_2_datetime(Some(r.created_at)).unwrap();
        let deleted_at = string_2_datetime(r.deleted_at);

        Ok(WortSchema {
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

    fn from_vec_raw(data: Vec<RawWortSchema>) -> Result<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
}
