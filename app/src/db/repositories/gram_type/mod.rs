use rusqlite::{Connection, Transaction};

use crate::{
    db::{
        schemas::gram_type::{InputGramType, SchemaGramType, SqlGramType},
        traits::{FromSql, SqlInsert},
    },
    helpers::error_handler::DbError,
};

#[cfg(test)]
mod gram_type_test;

pub struct GramTypeRepo;

impl GramTypeRepo {
    pub fn bulk_upsert(
        conn: &mut Connection,
        data: &[InputGramType],
    ) -> Result<Vec<SchemaGramType>, DbError> {
        let tx = conn.transaction()?;
        let out = Self::bulk_upsert_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    pub fn bulk_upsert_tx(
        tx: &Transaction,
        data: &[InputGramType],
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

        let mut stmt = tx.prepare_cached(sql).map_err(DbError::with_sql(sql))?;

        let mut vec_out = Vec::with_capacity(data.len());

        for d in data {
            let params: SqlGramType = d.to_owned().into();
            let raw = stmt
                .query_one(params.insert_params(), SchemaGramType::from_sql)
                .map_err(DbError::with_sql(sql))?;

            vec_out.push(raw);
        }

        Ok(vec_out)
    }
}
