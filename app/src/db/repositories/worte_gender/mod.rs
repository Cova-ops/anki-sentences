use color_eyre::eyre::{Context, Result};
use rusqlite::{Connection, Transaction};
use sql_model::{FromRaw, SqlNew, SqlRaw};

use crate::db::schemas::worte_gender::{
    NewWorteGenderSchema as New, RawWorteGenderSchema as Raw, WorteGenderSchema as Schema,
};

#[cfg(test)]
mod worte_gender_test;

pub struct WorteGenderRepo;

impl WorteGenderRepo {
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
            INSERT INTO worte_gender (id, gender, artikel)
                VALUES (?1, ?2, ?3) ON CONFLICT(id) DO UPDATE SET gender = ?2, artikel = ?3
            RETURNING id, gender, artikel, created_at, deleted_at;
        "#;

        let mut vec_out = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare_cached(sql)?;

        for d in data {
            let raw = stmt
                .query_one(d.to_params(), Raw::from_sql)
                .with_context(|| format!("sql: {}, params: {:#?}", sql, d))?;
            vec_out.push(Schema::from_raw(raw)?);
        }

        Ok(vec_out)
    }
}
