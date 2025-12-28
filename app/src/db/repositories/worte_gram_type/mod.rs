use color_eyre::eyre::{Context, Result};
use rusqlite::{Connection, Transaction, params, params_from_iter};
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
            let raw = stmt
                .query_one(d.to_params(), Raw::from_sql)
                .context(format!("sql: {}, params: {:#?}", sql, d))?;
            vec_out.push(Schema::from_raw(raw)?)
        }

        Ok(vec_out)
    }

    // TODO: HAcer el test
    pub fn fetch_by_wort_id(conn: &Connection, ids: &[i32]) -> Result<Vec<Schema>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let placeholders: String = std::iter::repeat_n("?", ids.len())
            .collect::<Vec<_>>()
            .join(",");

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

        let params: Vec<&dyn rusqlite::ToSql> =
            ids.iter().map(|t| t as &dyn rusqlite::ToSql).collect();

        let mut stmt = conn.prepare(&sql)?;
        let raw = stmt
            .query(params_from_iter(params))?
            .mapped(Raw::from_sql)
            .collect::<Result<Vec<Raw>, _>>()?;

        let out = Schema::from_vec_raw(raw)?;
        Ok(out)
    }

    pub fn fetch_all_ids(conn: &Connection, limit: usize, last_id: i32) -> Result<Vec<i32>> {
        let sql = r#"
            SELECT id_worte
            FROM worte_gram_type wgt
            WHERE wgt.deleted_at is NULL AND wgt.id_worte > ?1
            ORDER BY wgt.id_worte
            LIMIT ?2;
        "#;

        let mut stmt = conn.prepare(&sql)?;
        let vec_ids = stmt
            .query(params![last_id as i64, limit as i64])?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(vec_ids)
    }

    pub fn delete_by_id(conn: &Connection, ids: &[i32]) -> Result<usize> {
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

        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.execute(params_from_iter(ids))?;

        Ok(rows)
    }
}
