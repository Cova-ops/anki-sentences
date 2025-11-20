use color_eyre::eyre::Result;
use rusqlite::{Connection, Transaction, params};
use sql_model::{FromRaw, SqlRaw};

use crate::db::schemas::gender_worte::{
    GenderWorteSchema as Schema, NewGenderWorteSchema as New, RawGenderWorteSchema as Raw,
};

pub struct GenderWorteRepo;

impl GenderWorteRepo {
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
            INSERT INTO gender_worte (id, gender, artikel)
                VALUES (?1, ?2, ?3) ON CONFLICT(id) DO UPDATE SET gender = ?2, artikel = ?3
            RETURNING id,gender,artikel,created_at,deleted_at;
        "#;

        let mut vec_out = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare_cached(sql)?;

        for d in data {
            let raw = stmt.query_one(params![d.id, d.gender, d.artikel], Raw::from_sql)?;

            vec_out.push(Schema::from_raw(raw)?);
        }

        Ok(vec_out)
    }
}
