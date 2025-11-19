use color_eyre::eyre::Result;

use crate::{
    db::{
        schemas::worte_gram_type::{
            NewWorteGramTypeSchema, RawWorteGramTypeSchema, WorteGramTypeSchema,
        },
        traits::FromRaw,
    },
    helpers::time::string_2_datetime,
};

impl NewWorteGramTypeSchema {
    pub fn new(id_worte: i32, id_gram_type: i32) -> Self {
        Self {
            id_worte,
            id_gram_type,
        }
    }
}

impl FromRaw<RawWorteGramTypeSchema> for WorteGramTypeSchema {
    fn from_raw(r: RawWorteGramTypeSchema) -> Result<Self> {
        let created_at = string_2_datetime(Some(r.created_at)).unwrap();
        let deleted_at = string_2_datetime(r.deleted_at);

        Ok(WorteGramTypeSchema {
            id_worte: r.id_worte,
            id_gram_type: r.id_gram_type,
            created_at,
            deleted_at,
        })
    }

    fn from_vec_raw(data: Vec<RawWorteGramTypeSchema>) -> Result<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
}
