use color_eyre::eyre::Result;

use crate::{
    db::{
        schemas::geschichtlich_setze::{GeschichtlichSetzeSchema, RawGeschichtlichSetzeSchema},
        traits::FromRaw,
    },
    helpers::time::string_2_datetime,
};

impl FromRaw<RawGeschichtlichSetzeSchema> for GeschichtlichSetzeSchema {
    fn from_raw(r: RawGeschichtlichSetzeSchema) -> Result<Self> {
        let created_at = string_2_datetime(Some(r.created_at)).unwrap();
        let deleted_at = string_2_datetime(r.deleted_at);

        Ok(GeschichtlichSetzeSchema {
            id: r.id,
            setze_id: r.setze_id,
            result: r.result != 0,
            created_at,
            deleted_at,
        })
    }

    fn from_vec_raw(data: Vec<RawGeschichtlichSetzeSchema>) -> Result<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
}
