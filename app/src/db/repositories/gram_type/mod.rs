use rusqlite::{Connection, Transaction};

use crate::{
    db::{
        queries::DbQuery,
        schemas::gram_type::{InputGramType, SchemaGramType, SqlGramType},
        traits::{FromSql, SqlNew},
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

        let mut stmt = tx.prepare_cached(sql)?;
        let mut vec_out = Vec::with_capacity(data.len());

        for d in data {
            let sql_params: SqlGramType = d.to_owned().into();
            let raw = DbQuery::query_one(&tx, sql, sql_params.to_params(), |r| {
                SchemaGramType::from_sql(r)
            })?;
            vec_out.push(raw);
        }

        Ok(vec_out)
    }
}
