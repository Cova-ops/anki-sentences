use chrono::{DateTime, Utc};
use rusqlite::{Connection, Transaction, params, params_from_iter};

use crate::{
    db::{
        schemas::setze_review::{InputSetzeReview, SchemaSetzeReview, SqlSetzeReview},
        traits::{FromSql, SqlInsert},
    },
    helpers::{error_handler::DbError, time::datetime_2_string},
};

#[cfg(test)]
mod setze_review_test;

pub struct SetzeReviewRepo;

impl SetzeReviewRepo {
    pub fn bulk_upsert(
        conn: &mut Connection,
        data: &[InputSetzeReview],
    ) -> Result<Vec<SchemaSetzeReview>, DbError> {
        let tx = conn.transaction()?;
        let out = Self::bulk_upsert_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    pub fn bulk_upsert_tx(
        tx: &Transaction,
        data: &[InputSetzeReview],
    ) -> Result<Vec<SchemaSetzeReview>, DbError> {
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
        let mut stmt = tx.prepare_cached(sql).map_err(DbError::with_sql(sql))?;
        for d in data {
            let params: SqlSetzeReview = d.to_owned().into();
            let raw = stmt
                .query_one(
                    params_from_iter(params.insert_params()),
                    SchemaSetzeReview::from_sql,
                )
                .map_err(DbError::with_sql(sql))?;

            vec_out.push(raw)
        }

        Ok(vec_out)
    }

    pub fn fetch_by_satz_id(
        conn: &Connection,
        ids: &[i32],
    ) -> Result<Vec<SchemaSetzeReview>, DbError> {
        let placeholders = vec!["?"; ids.len()].join(",");

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

        let mut stmt = conn.prepare(&sql).map_err(DbError::with_sql(&sql))?;
        let raw = stmt
            .query(params_from_iter(ids))
            .map_err(DbError::with_sql(&sql))?
            .mapped(SchemaSetzeReview::from_sql)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DbError::with_sql(&sql))?;

        Ok(raw)
    }

    pub fn fetch_review_satz_id_by_day(
        conn: &Connection,
        date_review: DateTime<Utc>,
    ) -> Result<Vec<i32>, DbError> {
        let sql = r#"
            SELECT satz_id
            FROM setze_review
            WHERE next_review < ?1
                AND deleted_at IS NULL
            ORDER BY next_review ASC;
        "#;

        let date = datetime_2_string(date_review);

        let mut stmt = conn.prepare(sql).map_err(DbError::with_sql(sql))?;
        let vec_ids: Vec<i32> = stmt
            .query(params![date])
            .map_err(DbError::with_sql(sql))?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<_>, _>>()
            .map_err(DbError::with_sql(sql))?;

        Ok(vec_ids)
    }
}
