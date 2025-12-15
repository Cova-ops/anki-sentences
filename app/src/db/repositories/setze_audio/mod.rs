use color_eyre::eyre::Result;
use rusqlite::{Connection, Transaction, params_from_iter};
use sql_model::{FromRaw, SqlNew, SqlRaw};

use crate::db::schemas::setze_audio::{
    NewSetzeAudioSchema as New, RawSetzeAudioSchema as Raw, SetzeAudioSchema as Schema,
};

#[cfg(test)]
mod setze_audio_test;

pub struct SetzeAudioRepo;

impl SetzeAudioRepo {
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
            INSERT INTO setze_audio (satz_id, file_path, voice_id)
                VALUES (?1, ?2, ?3)
            ON CONFLICT(satz_id) DO UPDATE SET file_path = ?2, voice_id = ?3
            RETURNING satz_id, file_path, voice_id, created_at, deleted_at;
            "#;

        let mut vec_out = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare_cached(sql)?;

        for d in data {
            let raw = stmt.query_one(d.to_params(), Raw::from_sql)?;
            vec_out.push(Schema::from_raw(raw)?);
        }

        Ok(vec_out)
    }

    pub fn fetch_by_id(conn: &Connection, ids: &[i32]) -> Result<Vec<Schema>> {
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

        let mut stmt = conn.prepare(&sql)?;
        let raw = stmt
            .query(params_from_iter(ids.iter()))?
            .mapped(Raw::from_sql)
            .collect::<Result<Vec<Raw>, _>>()?;

        let vec_out = Schema::from_vec_raw(raw)?;
        Ok(vec_out)
    }
}
