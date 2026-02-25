use rusqlite::{Connection, Transaction, params_from_iter};

use crate::{
    db::{
        schemas::setze_audio::{InputSetzeAudio, SchemaSetzeAudio, SqlSetzeAudio},
        traits::{FromSql, SqlInsert},
    },
    helpers::error_handler::DbError,
};

#[cfg(test)]
mod setze_audio_test;

pub struct SetzeAudioRepo;

impl SetzeAudioRepo {
    pub fn bulk_upsert(
        conn: &mut Connection,
        data: &[InputSetzeAudio],
    ) -> Result<Vec<SchemaSetzeAudio>, DbError> {
        let tx = conn.transaction()?;
        let out = Self::bulk_upsert_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    pub fn bulk_upsert_tx(
        tx: &Transaction,
        data: &[InputSetzeAudio],
    ) -> Result<Vec<SchemaSetzeAudio>, DbError> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            INSERT INTO setze_audio (satz_id, file_path, voice_id)
                VALUES (?1, ?2, ?3)
            ON CONFLICT(satz_id) DO UPDATE SET file_path = ?2, voice_id = ?3
            RETURNING satz_id, file_path, voice_id, created_at, deleted_at;
            "#;

        let mut vec_out = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare_cached(sql).map_err(DbError::with_sql(sql))?;

        for d in data {
            let params: SqlSetzeAudio = d.to_owned().into();
            let raw = stmt
                .query_one(
                    params_from_iter(params.insert_params()),
                    SchemaSetzeAudio::from_sql,
                )
                .map_err(DbError::with_sql(sql))?;

            vec_out.push(raw);
        }

        Ok(vec_out)
    }

    pub fn fetch_by_id(conn: &Connection, ids: &[i32]) -> Result<Vec<SchemaSetzeAudio>, DbError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let placeholders = vec!["?"; ids.len()].join(",");
        let sql = format!(
            "
            SELECT satz_id, file_path, voice_id, created_at, deleted_at
            FROM setze_audio
            WHERE satz_id in ({placeholders})
                AND deleted_at is NULL
            ORDER BY satz_id;
        "
        );

        let mut stmt = conn.prepare(&sql).map_err(DbError::with_sql(&sql))?;
        let raw = stmt
            .query(params_from_iter(ids.iter()))
            .map_err(DbError::with_sql(&sql))?
            .mapped(SchemaSetzeAudio::from_sql)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(raw)
    }
}
