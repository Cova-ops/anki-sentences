use color_eyre::eyre::Result;
use rusqlite::{Connection, Transaction, params};
use sql_model::{FromRaw, SqlRaw};

use crate::db::schemas::niveau_worte::{
    NewNiveauWorteSchema as New, NiveauWorteSchema as Schema, RawNiveauWorteSchema as Raw,
};

pub struct NiveauWorteRepo;

impl NiveauWorteRepo {
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
            INSERT INTO niveau_worte (id, niveau)
                VALUES (?1, ?2)
            ON CONFLICT(id) DO UPDATE SET niveau = ?2
            RETURNING id,  niveau, created_at, deleted_at;
        "#;

        let mut vec_out = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare_cached(sql)?;

        for d in data {
            let raw = stmt.query_one(params![d.id, d.niveau], Raw::from_sql)?;
            vec_out.push(Schema::from_raw(raw)?)
        }

        Ok(vec_out)
    }
}
