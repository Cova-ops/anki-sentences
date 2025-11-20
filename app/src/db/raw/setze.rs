use color_eyre::eyre::{Context, Result};
use sql_model::FromRaw;

use crate::{
    ctx,
    db::schemas::{
        schwirigkeit_liste::SchwirigkeitListeSchema,
        setze::{NewSetzeSchema as New, RawSetzeSchema as Raw, SetzeSchema as Schema},
    },
    helpers::time::string_2_datetime,
};

impl FromRaw<Raw> for Schema {
    fn from_raw(r: Raw) -> Result<Self> {
        let created_at = string_2_datetime(Some(r.created_at)).unwrap();
        let deleted_at = string_2_datetime(r.deleted_at);

        let schwirig_id = SchwirigkeitListeSchema::from_id(r.schwirig_id).context(ctx!())?;

        Ok(Schema {
            id: r.id,
            setze_spanisch: r.setze_spanisch,
            setze_deutsch: r.setze_deutsch,
            thema: r.thema,
            schwirig_id,
            created_at,
            deleted_at,
        })
    }

    fn from_vec_raw(data: Vec<Raw>) -> Result<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
}

impl New {
    pub fn new(
        setze_spanisch: String,
        setze_deutsch: String,
        thema: String,
        schwirig_id: i32,
    ) -> Self {
        Self {
            setze_spanisch,
            setze_deutsch,
            thema,
            schwirig_id,
        }
    }
}
