use rusqlite::{Connection, Transaction, params, params_from_iter};

use crate::{
    db::{
        schemas::wort_audio::{InputWortAudio, SchemaWortAudio, SqlWortAudio},
        traits::{FromSql, SqlInsert},
        views::wort_audio_missing::SchemaWortAudioMissing,
    },
    helpers::error_handler::DbError,
};

#[cfg(test)]
mod wort_audio_test;

pub struct WortAudioRepo;

impl WortAudioRepo {
    pub fn bulk_upsert(
        conn: &mut Connection,
        data: &[InputWortAudio],
    ) -> Result<Vec<SchemaWortAudio>, DbError> {
        let tx = conn.transaction()?;
        let out = Self::bulk_upsert_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    pub fn bulk_upsert_tx(
        tx: &Transaction,
        data: &[InputWortAudio],
    ) -> Result<Vec<SchemaWortAudio>, DbError> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            INSERT INTO worte_audio (wort_id, audio_name_es, audio_name_de)
                VALUES (?1, ?2, ?3)
            ON CONFLICT(wort_id) DO UPDATE SET audio_name_es = ?2, audio_name_de = ?3
            RETURNING wort_id, audio_name_es, audio_name_de, created_at, deleted_at;
            "#;

        let mut vec_out = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare_cached(sql).map_err(DbError::with_sql(sql))?;

        for d in data {
            let params: SqlWortAudio = d.to_owned().into();
            let raw = stmt
                .query_one(
                    params_from_iter(params.insert_params()),
                    SchemaWortAudio::from_sql,
                )
                .map_err(DbError::with_sql(sql))?;

            vec_out.push(raw);
        }

        Ok(vec_out)
    }

    pub fn fetch_by_id(conn: &Connection, ids: &[i32]) -> Result<Vec<SchemaWortAudio>, DbError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let placeholders = vec!["?"; ids.len()].join(",");
        let sql = format!(
            "
            SELECT
                wort_id,
                audio_name_es,
                audio_name_de,
                created_at,
                deleted_at
            FROM worte_audio
            WHERE wort_id in ({placeholders})
                AND deleted_at is NULL
            ORDER BY wort_id;
        "
        );

        let mut stmt = conn.prepare(&sql).map_err(DbError::with_sql(&sql))?;
        let raw = stmt
            .query(params_from_iter(ids.iter()))
            .map_err(DbError::with_sql(&sql))?
            .mapped(SchemaWortAudio::from_sql)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DbError::with_sql(&sql))?;

        Ok(raw)
    }

    pub fn fetch_all_ids(
        conn: &Connection,
        limit: usize,
        last_id: i32,
    ) -> Result<Vec<i32>, DbError> {
        let sql = r#"
            SELECT wort_id
            FROM worte_audio wa
            WHERE wa.deleted_at is NULL AND wa.wort_id > ?1
            ORDER BY wa.wort_id
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

    pub fn fetch_worte_without_audio(
        conn: &Connection,
    ) -> Result<Vec<SchemaWortAudioMissing>, DbError> {
        let sql = "
            SELECT
                w.id,
                w.wort_es,
                w.wort_de,
                wa.audio_name_es,
                wa.audio_name_de
            FROM worte w
            LEFT JOIN worte_audio wa ON w.id = wa.wort_id 
            WHERE w.deleted_at IS NULL AND (wa.audio_name_es is NULL OR wa.audio_name_de is NULL)
            ORDER BY w.id ASC;";

        let mut stmt = conn.prepare_cached(sql).map_err(DbError::with_sql(sql))?;

        let rows = stmt
            .query([])
            .map_err(DbError::with_sql(sql))?
            .mapped(SchemaWortAudioMissing::from_sql)
            .collect::<Result<Vec<_>, _>>()
            .map_err(DbError::with_sql(sql))?;

        Ok(rows)
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
                DELETE FROM worte_audio
                WHERE wort_id IN ({placeholder});
            "
        );

        let mut stmt = tx.prepare(&sql).map_err(DbError::with_sql(&sql))?;
        let rows_afected = stmt
            .execute(params_from_iter(ids))
            .map_err(DbError::with_sql(&sql))?;

        Ok(rows_afected)
    }
}
