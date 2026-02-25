use rusqlite::{Connection, Transaction, params_from_iter};

use crate::{
    db::{
        schemas::wort_gender::{InputWortGender, SchemaWortGender, SqlWortGender},
        traits::{FromSql, SqlInsert},
    },
    helpers::error_handler::DbError,
};

#[cfg(test)]
mod wort_gender_test;

pub struct WortGenderRepo;

impl WortGenderRepo {
    pub fn bulk_upsert(
        conn: &mut Connection,
        data: &[InputWortGender],
    ) -> Result<Vec<SchemaWortGender>, DbError> {
        let tx = conn.transaction()?;
        let out = Self::bulk_upsert_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    pub fn bulk_upsert_tx(
        tx: &Transaction,
        data: &[InputWortGender],
    ) -> Result<Vec<SchemaWortGender>, DbError> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            INSERT INTO worte_gender (id, gender, artikel)
                VALUES (?1, ?2, ?3)
            ON CONFLICT(id) DO UPDATE SET 
                gender = ?2,
                artikel = ?3
            RETURNING gender, created_at, deleted_at;
        "#;

        let mut vec_out = Vec::with_capacity(data.len());
        let mut stmt = tx.prepare_cached(sql).map_err(DbError::with_sql(sql))?;

        for d in data {
            let params: SqlWortGender = d.to_owned().into();
            let raw = stmt
                .query_one(
                    params_from_iter(params.insert_params()),
                    SchemaWortGender::from_sql,
                )
                .map_err(DbError::with_sql(sql))?;

            vec_out.push(raw);
        }

        Ok(vec_out)
    }
}
