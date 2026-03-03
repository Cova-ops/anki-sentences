use chrono::{DateTime, Utc};
use rusqlite::{Connection, Transaction, params, params_from_iter};

use crate::{
    db::{
        schemas::wort_review::{
            EnumReviewDirection, InputWortReview, SchemaWortReview, SqlWortReview,
        },
        traits::{FromSql, SqlInsert},
    },
    helpers::{error_handler::DbError, time::datetime_2_string},
};

#[cfg(test)]
mod wort_review_test;

pub struct WortReviewRepo;

impl WortReviewRepo {
    pub fn bulk_upsert(
        conn: &mut Connection,
        data: &[InputWortReview],
    ) -> Result<Vec<SchemaWortReview>, DbError> {
        let tx = conn.transaction()?;
        let out = Self::bulk_upsert_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    pub fn bulk_upsert_tx(
        tx: &Transaction,
        data: &[InputWortReview],
    ) -> Result<Vec<SchemaWortReview>, DbError> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            INSERT INTO worte_review (
                wort_id,
                direction,
                interval,
                ease_factor,
                repetitions,
                last_review,
                next_review
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)

            ON CONFLICT(wort_id, direction)
            DO UPDATE SET
                interval      = excluded.interval,
                ease_factor   = excluded.ease_factor,
                repetitions   = excluded.repetitions,
                last_review   = excluded.last_review,
                next_review   = excluded.next_review

            RETURNING
                wort_id,
                direction,
                interval,
                ease_factor,
                repetitions,
                last_review,
                next_review,
                created_at,
                deleted_at;
        "#;

        let mut vec_out = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare_cached(sql).map_err(DbError::with_sql(sql))?;
        for d in data {
            let params: SqlWortReview = d.to_owned().into();
            let raw = stmt
                .query_one(
                    params_from_iter(params.insert_params()),
                    SchemaWortReview::from_sql,
                )
                .map_err(DbError::with_sql(sql))?;

            vec_out.push(raw)
        }

        Ok(vec_out)
    }

    pub fn fetch_by_wort_id(
        conn: &Connection,
        ids: &[i32],
    ) -> Result<Vec<SchemaWortReview>, DbError> {
        let placeholders = vec!["?"; ids.len()].join(",");

        let sql = format!(
            "
                SELECT 
                    wort_id,
                    direction,
                    interval,
                    ease_factor,
                    repetitions,
                    last_review,
                    next_review,
                    created_at,
                    deleted_at
                FROM 
                    worte_review wr
                WHERE 
                    wr.deleted_at is NULL
                    AND wr.wort_id in ({placeholders})
                ORDER BY
                    wr.wort_id ASC;
            "
        );

        let mut stmt = conn.prepare(&sql).map_err(DbError::with_sql(&sql))?;
        let raw = stmt
            .query(params_from_iter(ids))
            .map_err(DbError::with_sql(&sql))?
            .mapped(SchemaWortReview::from_sql)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DbError::with_sql(&sql))?;

        Ok(raw)
    }

    pub fn fetch_new_wort_id_4_review(
        conn: &Connection,
        lang: EnumReviewDirection,
    ) -> Result<Vec<i32>, DbError> {
        let sql = format!(
            r#"
                SELECT
                    w.id
                FROM worte w
                WHERE NOT EXISTS (
                    SELECT 1
                    FROM worte_review wr
                    WHERE 
                        wr.wort_id = w.id
                        AND direction = "{}"
                )
                AND w.deleted_at IS NULL
                ORDER BY w.id ASC;
            "#,
            lang.as_str()
        );

        let mut stmt = conn.prepare(&sql).map_err(DbError::with_sql(&sql))?;
        let vec_ids: Vec<i32> = stmt
            .query([])
            .map_err(DbError::with_sql(&sql))?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<_>, _>>()
            .map_err(DbError::with_sql(&sql))?;

        Ok(vec_ids)
    }

    pub fn fetch_review_wort_id_by_day(
        conn: &Connection,
        date_review: DateTime<Utc>,
        lang: EnumReviewDirection,
    ) -> Result<Vec<i32>, DbError> {
        let sql = r#"
                SELECT wort_id
                FROM worte_review
                WHERE next_review < ?1
                    AND direction = ?2
                    AND deleted_at IS NULL
                ORDER BY next_review ASC;
            "#;

        let mut stmt = conn.prepare(sql).map_err(DbError::with_sql(sql))?;

        let date = datetime_2_string(date_review);
        let vec_ids: Vec<i32> = stmt
            .query(params![date, lang.as_str()])
            .map_err(DbError::with_sql(sql))?
            .mapped(|r| r.get(0))
            .collect::<Result<_, _>>()
            .map_err(DbError::with_sql(sql))?;

        Ok(vec_ids)
    }

    pub fn fetch_all_ids(
        conn: &Connection,
        limit: usize,
        last_id: i32,
    ) -> Result<Vec<i32>, DbError> {
        let sql = r#"
            SELECT wort_id
            FROM worte_review wr
            WHERE wr.deleted_at is NULL AND wr.wort_id > ?1
            ORDER BY wr.wort_id
            LIMIT ?2;
        "#;

        let mut stmt = conn.prepare(sql).map_err(DbError::with_sql(sql))?;
        let vec_ids: Vec<i32> = stmt
            .query(params![last_id as i64, limit as i64])
            .map_err(DbError::with_sql(sql))?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<_>, _>>()
            .map_err(DbError::with_sql(sql))?;

        Ok(vec_ids)
    }

    pub fn delete_by_id(conn: &mut Connection, ids: &[i32]) -> Result<usize, DbError> {
        let tx = conn.transaction()?;
        let out = Self::delete_by_id_tx(&tx, ids)?;
        tx.commit()?;
        Ok(out)
    }

    pub fn delete_by_id_tx(tx: &Transaction, ids: &[i32]) -> Result<usize, DbError> {
        if ids.is_empty() {
            return Ok(0);
        }

        let placeholder = vec!["?"; ids.len()].join(",");

        let sql = format!(
            "
            DELETE FROM worte_review
            WHERE wort_id IN ({placeholder});
        "
        );

        let mut stmt = tx.prepare(&sql).map_err(DbError::with_sql(&sql))?;
        let rows = stmt
            .execute(params_from_iter(ids))
            .map_err(DbError::with_sql(&sql))?;

        Ok(rows)
    }
}
