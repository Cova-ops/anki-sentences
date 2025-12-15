use color_eyre::eyre::{Context, Result};
use rusqlite::{Connection, Transaction, params, params_from_iter};
use sql_model::{FromRaw, SqlNew, SqlRaw};

use crate::db::schemas::setze_review::{
    NewSetzeReviewSchema as New, RawWorteReviewSchema as Raw, SetzeReviewSchema as Schema,
};

#[cfg(test)]
mod setze_review_test;

pub struct SetzeReviewRepo;

impl SetzeReviewRepo {
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
            INSERT INTO setze_review (satz_id, interval, ease_factor, repetitions, last_review, next_review)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)

            ON CONFLICT(satz_id) DO UPDATE SET 
                interval = ?2,
                ease_factor = ?3,
                repetitions = ?4,
                last_review = ?5,
                next_review = ?6
            
            RETURNING id, satz_id, interval, ease_factor, repetitions, last_review, next_review, created_at, deleted_at;
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

    pub fn fetch_by_satz_id(conn: &Connection, ids: &[i32]) -> Result<Vec<Schema>> {
        let placeholders = std::iter::repeat_n("?", ids.len())
            .collect::<Vec<_>>()
            .join(",");

        let params: Vec<&dyn rusqlite::ToSql> =
            ids.iter().map(|t| t as &dyn rusqlite::ToSql).collect();

        let sql = format!(
            "
                SELECT 
                    id, satz_id, interval, ease_factor, repetitions,
                    last_review, next_review, created_at, deleted_at
                FROM setze_review sr
                WHERE sr.deleted_at is NULL AND
                sr.satz_id in ({placeholders})
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

    pub fn fetch_review_satz_id_by_day(conn: &Connection, date_review: String) -> Result<Vec<i32>> {
        let sql = r#"
            SELECT satz_id
            FROM setze_review
            WHERE next_review < ?1
                AND deleted_at IS NULL
            ORDER BY next_review ASC;
        "#;

        let mut stmt = conn.prepare(sql)?;
        let vec_ids = stmt
            .query(params![date_review])?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(vec_ids)
    }
}
