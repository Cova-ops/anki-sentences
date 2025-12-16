use color_eyre::eyre::Result;
use sql_model::FromRaw;

use crate::{
    db::schemas::setze_review::{RawWorteReviewSchema as Raw, SetzeReviewSchema as Schema},
    helpers::time::string_2_datetime,
};

impl FromRaw<Raw> for Schema {
    fn from_raw(r: Raw) -> Result<Self> {
        let last_review = string_2_datetime(Some(r.last_review)).unwrap();
        let next_review = string_2_datetime(Some(r.next_review)).unwrap();

        let created_at = string_2_datetime(Some(r.created_at)).unwrap();
        let deleted_at = string_2_datetime(r.deleted_at);

        Ok(Schema {
            id: r.id,

            satz_id: r.satz_id,
            interval: r.interval,
            ease_factor: r.ease_factor,
            repetitions: r.repetitions,
            last_review,
            next_review,

            created_at,
            deleted_at,
        })
    }

    fn from_vec_raw(data: Vec<Raw>) -> Result<Vec<Self>> {
        data.into_iter().map(Self::from_raw).collect()
    }
}
