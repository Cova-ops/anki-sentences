use rusqlite::{Connection, Transaction, params, params_from_iter};

use crate::{
    db::{
        schemas::wort_gram_type::{InputWortGramType, SchemaWortGramType, SqlWortGramType},
        traits::{FromSql, SqlNew},
    },
    helpers::error_handler::DbError,
};

#[cfg(test)]
mod wort_gram_type_test;

pub struct WortGramTypeRepo;

impl WortGramTypeRepo {
    pub fn bulk_insert(
        conn: &mut Connection,
        data: &[InputWortGramType],
    ) -> Result<Vec<SchemaWortGramType>, DbError> {
        let tx = conn.transaction()?;
        let out = Self::bulk_insert_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    pub fn bulk_insert_tx(
        tx: &Transaction,
        data: &[InputWortGramType],
    ) -> Result<Vec<SchemaWortGramType>, DbError> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            INSERT INTO worte_gram_type (id_worte, id_gram_type)
                VALUES (?1, ?2)
            RETURNING id_worte, id_gram_type, created_at, deleted_at;
        "#;

        let mut vec_out = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare(sql).map_err(DbError::with_sql(sql))?;

        for d in data {
            let params: SqlWortGramType = d.to_owned().into();
            let raw = stmt
                .query_one(params.to_params(), SchemaWortGramType::from_sql)
                .map_err(DbError::with_sql(sql))?;

            vec_out.push(raw);
        }

        Ok(vec_out)
    }

    pub fn fetch_by_wort_id(
        conn: &Connection,
        ids: &[i32],
    ) -> Result<Vec<SchemaWortGramType>, DbError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let placeholders: String = vec!["?"; ids.len()].join(",");

        let sql = format!(
            "
            SELECT 
                id_worte, id_gram_type, created_at, deleted_at
            FROM worte_gram_type wgt
            WHERE wgt.deleted_at is NULL AND
            wgt.id_worte in ({placeholders})
            ORDER BY wgt.id_worte;
        "
        );

        let mut stmt = conn.prepare(&sql).map_err(DbError::with_sql(&sql))?;
        let raw = stmt
            .query(params_from_iter(ids))
            .map_err(DbError::with_sql(&sql))?
            .mapped(SchemaWortGramType::from_sql)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DbError::with_sql(&sql))?;

        Ok(raw)
    }

    pub fn fetch_all_worte_id(
        conn: &Connection,
        limit: usize,
        last_id: i32,
    ) -> Result<Vec<i32>, DbError> {
        let sql = r#"
            SELECT DISTINCT id_worte
            FROM worte_gram_type wgt
            WHERE wgt.deleted_at is NULL AND wgt.id_worte > ?1
            ORDER BY wgt.id_worte
            LIMIT ?2;
        "#;

        let mut stmt = conn.prepare(&sql).map_err(DbError::with_sql(sql))?;
        let vec_ids = stmt
            .query(params![last_id as i64, limit as i64])
            .map_err(DbError::with_sql(sql))?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<i32>, _>>()
            .map_err(DbError::with_sql(sql))?;

        Ok(vec_ids)
    }

    pub fn delete_by_wort_id(conn: &mut Connection, ids: &[i32]) -> Result<usize, DbError> {
        let tx = conn.transaction()?;
        let out = Self::delete_by_wort_id_tx(&tx, ids)?;
        tx.commit()?;
        Ok(out)
    }

    pub fn delete_by_wort_id_tx(tx: &Transaction, ids: &[i32]) -> Result<usize, DbError> {
        if ids.is_empty() {
            return Ok(0);
        }

        let placeholder = vec!["?"; ids.len()].join(",");

        let sql = format!(
            "
            DELETE FROM worte_gram_type
            WHERE id_worte IN ({placeholder});
        "
        );

        let mut stmt = tx.prepare_cached(&sql).map_err(DbError::with_sql(&sql))?;
        let rows = stmt
            .execute(params_from_iter(ids))
            .map_err(DbError::with_sql(&sql))?;

        Ok(rows)
    }
}
