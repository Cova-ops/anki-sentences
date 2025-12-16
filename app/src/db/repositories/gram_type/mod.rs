use color_eyre::eyre::Result;
use rusqlite::{Connection, Transaction, params};
use sql_model::{FromRaw, SqlNew, SqlRaw};

use crate::db::schemas::gram_type::{
    GramTypeSchema as Schema, NewGramTypeSchema as New, RawGramTypeSchema as Raw,
};

#[cfg(test)]
mod gram_type_test;

pub struct GramTypeRepo;

impl GramTypeRepo {
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
            INSERT INTO gram_type (id, code, name)
                VALUES (?1, ?2, ?3) 
            ON CONFLICT(id) DO UPDATE SET code = ?2, name = ?3
            ON CONFLICT(code) DO UPDATE SET name = ?3
            RETURNING id, code, name, created_at, deleted_at;
        "#;

        let mut stmt = tx.prepare_cached(sql)?;
        let mut vec_out = Vec::with_capacity(data.len());

        for d in data {
            let raw = stmt.query_one(d.to_params(), Raw::from_sql)?;
            vec_out.push(Schema::from_raw(raw)?);
        }

        Ok(vec_out)
    }
}
