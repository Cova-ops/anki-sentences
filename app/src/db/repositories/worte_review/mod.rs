use color_eyre::eyre::{Context, Result};
use rusqlite::{Connection, Transaction, params_from_iter};
use sql_model::{FromRaw, SqlNew, SqlRaw};

use crate::db::schemas::worte_review::{
    NewWorteReviewSchema as New, RawWorteReviewSchema as Raw, WorteReviewSchema as Schema,
};

#[cfg(test)]
mod worte_review_test;

pub struct WorteReviewRepo;

impl WorteReviewRepo {
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
            INSERT INTO worte_review (wort_id, interval, ease_factor, repetitions, last_review, next_review)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)

            ON CONFLICT(wort_id) DO UPDATE SET 
                interval = ?2,
                ease_factor = ?3,
                repetitions = ?4,
                last_review = ?5,
                next_review = ?6
            
            RETURNING id, wort_id, interval, ease_factor, repetitions, last_review, next_review, created_at, deleted_at;
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

    pub fn fetch_by_wort_id(conn: &Connection, ids: &[i32]) -> Result<Vec<Schema>> {
        let placeholders = std::iter::repeat_n("?", ids.len())
            .collect::<Vec<_>>()
            .join(",");

        let params: Vec<&dyn rusqlite::ToSql> =
            ids.iter().map(|t| t as &dyn rusqlite::ToSql).collect();

        let sql = format!(
            "
                SELECT 
                    id, wort_id, interval, ease_factor, repetitions,
                    last_review, next_review, created_at, deleted_at
                FROM worte_review wr
                WHERE wr.deleted_at is NULL AND
                wr.wort_id in ({placeholders})
            "
        );

        let mut stmt = conn.prepare(&sql)?;
        let raw = stmt
            .query(params_from_iter(params))?
            .mapped(Raw::from_sql)
            .collect::<Result<Vec<Raw>, _>>()?;

        let vec_out = Schema::from_vec_raw(raw)?;
        Ok(vec_out)
    }
}
