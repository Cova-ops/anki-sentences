use color_eyre::eyre::{Context, Result};

use crate::{
    ctx,
    db::{
        schemas::{
            schwirigkeit_liste::SchwirigkeitListeSchema,
            setze::{NewSetzeSchema, RawSetzeSchema, SetzeSchema},
        },
        traits::FromRaw,
    },
    helpers::time::string_2_datetime,
};

impl NewSetzeSchema {
    pub fn new(
        setze_spanisch: impl Into<String>,
        setze_deutsch: impl Into<String>,
        thema: impl Into<String>,
        schwirig_id: impl Into<i32>,
    ) -> Self {
        Self {
            setze_spanisch: setze_spanisch.into(),
            setze_deutsch: setze_deutsch.into(),
            schwirig_id: schwirig_id.into(),
            thema: thema.into(),
        }
    }
}

impl FromRaw<RawSetzeSchema> for SetzeSchema {
    fn from_raw(r: RawSetzeSchema) -> Result<Self> {
        let created_at = string_2_datetime(Some(r.created_at)).unwrap();
        let deleted_at = string_2_datetime(r.deleted_at);

        let schwirig_id = SchwirigkeitListeSchema::from_id(r.schwirig_id_num).context(ctx!())?;

        Ok(SetzeSchema {
            id: r.id,
            setze_spanisch: r.setze_spanisch,
            setze_deutsch: r.setze_deutsch,
            thema: r.thema,
            schwirig_id,
            created_at,
            deleted_at,
        })
    }

    fn from_vec_raw(data: Vec<RawSetzeSchema>) -> Result<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
}
