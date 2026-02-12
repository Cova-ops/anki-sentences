use rusqlite::{Connection, Transaction};

use crate::{
    db::{
        schemas::niveau_liste::{InputNiveauListe, SchemaNiveauListe, SqlNiveauListe},
        traits::{FromSql, SqlNew},
    },
    helpers::error_handler::DbError,
};

#[cfg(test)]
mod niveau_liste_test;

pub struct NiveauListeRepo;
impl NiveauListeRepo {
    pub fn bulk_upsert(
        conn: &mut Connection,
        data: &[InputNiveauListe],
    ) -> Result<Vec<SchemaNiveauListe>, DbError> {
        let tx = conn.transaction()?;
        let out = Self::bulk_upsert_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    pub fn bulk_upsert_tx(
        tx: &Transaction,
        data: &[InputNiveauListe],
    ) -> Result<Vec<SchemaNiveauListe>, DbError> {
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
        for d in data {
            let params: SqlNiveauListe = d.to_owned().into();
            let raw = tx.query_one(sql, params.to_params(), SchemaNiveauListe::from_sql)?;
            vec_out.push(raw)
        }

        Ok(vec_out)
    }
}
