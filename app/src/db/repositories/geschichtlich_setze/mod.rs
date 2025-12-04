use color_eyre::eyre::Result;
use rusqlite::{Connection, Transaction};
use sql_model::{FromRaw, SqlNew, SqlRaw};

use crate::db::schemas::geschichtlich_setze::{
    GeschichtlichSetzeSchema as Schema, NewGeschichtlichSetzeSchema as New,
    RawGeschichtlichSetzeSchema as Raw,
};

#[cfg(test)]
mod geschichtlich_setze_test;

pub struct GeschichtlichSetzeRepo;

impl GeschichtlichSetzeRepo {
    #[cfg_attr(feature = "tested", doc = "v0.2")]
    pub fn bulk_insert(conn: &mut Connection, data: &[New]) -> Result<Vec<Schema>> {
        let tx = conn.transaction()?;
        let out = Self::bulk_insert_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    #[cfg_attr(feature = "tested", doc = "v0.2")]
    pub fn bulk_insert_tx(tx: &Transaction, data: &[New]) -> Result<Vec<Schema>> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            INSERT INTO geschichtlich_setze (setze_id, result)
                VALUES (?1,?2)
            RETURNING id,setze_id, result, created_at,deleted_at;
        "#;

        let mut vec_out = Vec::with_capacity(data.len());

        let mut stmt = tx.prepare_cached(sql)?;
        for d in data {
            let raw = stmt.query_row(d.to_params(), Raw::from_sql)?;
            vec_out.push(Schema::from_raw(raw)?);
        }

        Ok(vec_out)
    }
}
