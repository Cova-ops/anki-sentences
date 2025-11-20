use color_eyre::eyre::Result;
use rusqlite::{Connection, Transaction};
use sql_model::{FromRaw, SqlNew, SqlRaw};

use crate::db::schemas::schwirigkeit_liste::{
    NewSchwirigkeitListeSchema as New, RawSchwirigkeitListeSchema as Raw,
    SchwirigkeitListeSchema as Schema,
};

pub struct SchwirigkeitListeRepo;

impl SchwirigkeitListeRepo {
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
            INSERT INTO schwirigkeit_liste (id, schwirigkeit)
                VALUES (?1, ?2)
            ON CONFLICT(id) DO UPDATE SET schwirigkeit = ?2
            RETURNING id, schwirigkeit, created_at, deleted_at;
            "#;

        let mut vec_out = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare_cached(sql)?;

        for d in data {
            let raw = stmt.query_one(d.to_params(), Raw::from_sql)?;
            vec_out.push(Schema::from_raw(raw)?);
        }

        Ok(vec_out)
    }
}
