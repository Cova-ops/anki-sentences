use color_eyre::eyre::Result;

use crate::{
    db::{
        schemas::wort_gram_type::{
            NewWortGramTypeSchema, RawWortGramTypeSchema, WortGramTypeSchema,
        },
        traits::FromRaw,
    },
    helpers::time::string_2_datetime,
};

impl NewWortGramTypeSchema {
    pub fn new(id_wort: i32, id_gram_type: i32) -> Self {
        Self {
            id_wort,
            id_gram_type,
        }
    }
}

impl FromRaw<RawWortGramTypeSchema> for WortGramTypeSchema {
    fn from_raw(r: RawWortGramTypeSchema) -> Result<Self> {
        let created_at = string_2_datetime(Some(r.created_at)).unwrap();
        let deleted_at = string_2_datetime(r.deleted_at);

        Ok(WortGramTypeSchema {
            id_wort: r.id_wort,
            id_gram_type: r.id_gram_type,
            created_at,
            deleted_at,
        })
    }

    fn from_vec_raw(data: Vec<RawWortGramTypeSchema>) -> Result<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
}
