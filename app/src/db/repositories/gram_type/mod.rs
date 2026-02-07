use rusqlite::{Connection, Transaction};

use crate::{
    db::schemas::gram_type::{InsertGramType, SchemaGramType},
    helpers::error_handler::DbError,
};

#[cfg(test)]
mod gram_type_test;

pub struct GramTypeRepo;

impl GramTypeRepo {
    #[cfg_attr(feature = "tested", doc = "v0.2")]
    pub fn bulk_upsert(
        conn: &mut Connection,
        data: &[InsertGramType],
    ) -> Result<Vec<SchemaGramType>, DbError> {
        let tx = conn.transaction()?;
        let out = Self::bulk_upsert_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    #[cfg_attr(feature = "tested", doc = "v0.2")]
    pub fn bulk_upsert_tx(
        tx: &Transaction,
        data: &[InsertGramType],
    ) -> Result<Vec<SchemaGramType>, DbError> {
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
            let raw = stmt.query_one(d.to_params(), SchemaGramType::from_sql)?;
            vec_out.push(d);
        }

        Ok(vec_out)
    }
}
