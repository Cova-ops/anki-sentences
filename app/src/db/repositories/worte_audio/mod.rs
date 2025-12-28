use color_eyre::eyre::Result;
use rusqlite::{Connection, Transaction, params, params_from_iter};
use sql_model::{FromRaw, SqlNew, SqlRaw};

use crate::db::schemas::worte_audio::{
    NewWorteAudioSchema as New, RawWorteAudioSchema as Raw, WorteAudioSchema as Schema,
};

#[cfg(test)]
mod worte_audio_test;

pub struct WorteAudioRepo;

impl WorteAudioRepo {
    #[cfg_attr(feature = "tested", doc = "v0.2.1")]
    pub fn bulk_insert(conn: &mut Connection, data: &[New]) -> Result<Vec<Schema>> {
        let tx = conn.transaction()?;
        let out = Self::bulk_insert_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    #[cfg_attr(feature = "tested", doc = "v0.2.1")]
    pub fn bulk_insert_tx(tx: &Transaction, data: &[New]) -> Result<Vec<Schema>> {
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
            SELECT wort_id, audio_name_es, audio_name_de, created_at, deleted_at
            FROM worte_audio
            WHERE wort_id in ({placeholders})
                AND deleted_at is NULL
            ORDER BY wort_id;
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

    pub fn fetch_all_ids(conn: &Connection, limit: usize, last_id: i32) -> Result<Vec<i32>> {
        let sql = r#"
            SELECT wort_id
            FROM worte_audio wa
            WHERE wa.deleted_at is NULL AND wa.wort_id > ?1
            ORDER BY wa.wort_id
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
            DELETE FROM worte_audio
            WHERE wort_id IN ({placeholder});
        "
        );

        let mut stmt = conn.prepare(&sql)?;
        let rows_afected = stmt.execute(params_from_iter(ids))?;

        Ok(rows_afected)
    }
}
