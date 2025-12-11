use color_eyre::eyre::Result;
use rusqlite::{Connection, Transaction};
use sql_model::{FromRaw, SqlNew, SqlRaw};

use crate::db::schemas::niveau_liste::{
    NewNiveauListeSchema as New, NiveauListeSchema as Schema, RawNiveauListeSchema as Raw,
};

#[cfg(test)]
mod niveau_liste_test;

pub struct NiveauListeRepo;

impl NiveauListeRepo {
    #[cfg_attr(feature = "tested", doc = "v0.2")]
    pub fn bulk_insert(conn: &mut Connection, data: &[New]) -> Result<Vec<Schema>> {
        let tx = conn.transaction()?;
        let out = Self::bulk_insert_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    #[cfg_attr(feature = "tested", doc = "v0.2")]
    pub fn bulk_insert_tx(tx: &Transaction, data: &[New]) -> Result<Vec<Schema>> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            INSERT INTO niveau_liste (id, niveau)
                VALUES (?1, ?2)
            ON CONFLICT(id) DO UPDATE SET niveau = ?2
            RETURNING id,  niveau, created_at, deleted_at;
        "#;

        let mut vec_out = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare_cached(sql)?;

        for d in data {
            let raw = stmt.query_one(d.to_params(), Raw::from_sql)?;
            vec_out.push(Schema::from_raw(raw)?)
        }

        Ok(vec_out)
    }
}
