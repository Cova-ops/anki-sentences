use color_eyre::eyre::{Context, Result};
use rusqlite::{Connection, Transaction};
use sql_model::{FromRaw, SqlNew, SqlRaw};

use crate::db::schemas::worte_gram_type::{
    NewWorteGramTypeSchema as New, RawWorteGramTypeSchema as Raw, WorteGramTypeSchema as Schema,
};

#[cfg(test)]
mod worte_gram_type_test;

pub struct WorteGramTypeRepo;

impl WorteGramTypeRepo {
    pub fn bulk_insert(conn: &mut Connection, data: &[New]) -> Result<Vec<Schema>> {
        let tx = conn.transaction()?;
        let out = Self::bulk_insert_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    pub fn bulk_insert_tx(tx: &Transaction, data: &[New]) -> Result<Vec<Schema>> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            INSERT INTO worte_gram_type (id_worte, id_gram_type)
                VALUES (?1, ?2)
            
            RETURNING id_worte, id_gram_type, created_at, deleted_at;
        "#;

        let mut vec_out = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare_cached(sql).context(format!("sql: {}", sql))?;
        for d in data {
            println!("------------------------------ Dentro del loop: {:#?}", d);

            let raw = stmt
                .query_one(d.to_params(), Raw::from_sql)
                .context(format!("sql: {}, params: {:#?}", sql, d))?;
            vec_out.push(Schema::from_raw(raw)?)
        }

        Ok(vec_out)
    }
}
