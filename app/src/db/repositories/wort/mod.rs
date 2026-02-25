use std::collections::HashMap;

use rusqlite::{Connection, OptionalExtension, Transaction, params, params_from_iter};

use crate::{
    db::{
        schemas::{
            wort::{InputWort, SchemaWort, SqlWort},
            wort_gram_type::{InputWortGramType, SchemaWortGramType},
        },
        traits::{FromSql, SqlInsert, SqlUpdate},
        wort_gram_type::WortGramTypeRepo,
    },
    helpers::error_handler::DbError,
};

#[cfg(test)]
mod wort_test;

pub struct WortRepo;

impl WortRepo {
    fn hydrate_gram_types(
        conn: &Connection,
        worte: Vec<SchemaWort>,
    ) -> Result<Vec<(SchemaWort, Vec<SchemaWortGramType>)>, DbError> {
        if worte.is_empty() {
            return Ok(vec![]);
        }

        let ids: Vec<i32> = worte.iter().map(|w| w.id).collect();

        let worte_gram_type = WortGramTypeRepo::fetch_by_wort_id(conn, &ids)?;

        let mut map: HashMap<i32, Vec<SchemaWortGramType>> = HashMap::new();
        for wgt in worte_gram_type.into_iter() {
            map.entry(wgt.id_worte).or_default().push(wgt);
        }

        let vec_out: Vec<(SchemaWort, Vec<SchemaWortGramType>)> = worte
            .into_iter()
            .map(|d| {
                let id = d.id;

                (d, map.remove(&id).unwrap_or_default())
            })
            .collect();

        Ok(vec_out)
    }

    pub fn bulk_insert(
        conn: &mut Connection,
        data: &[InputWort],
    ) -> Result<Vec<(SchemaWort, Vec<SchemaWortGramType>)>, DbError> {
        let tx = conn.transaction()?;
        let out = Self::bulk_insert_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    pub fn bulk_insert_tx(
        tx: &Transaction,
        data: &[InputWort],
    ) -> Result<Vec<(SchemaWort, Vec<SchemaWortGramType>)>, DbError> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            INSERT INTO worte
                (
                    gender_id
                    wort_de
                    wort_es
                    plural
                    niveau_id
                    example_de
                    example_es
                    verb_aux
                    trennbar
                    reflexiv
                )
            VALUES
                (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            RETURNING
                id
                gender_id
                wort_de
                wort_es
                plural
                niveau_id
                example_de
                example_es
                verb_aux
                trennbar
                reflexiv
                created_at
                deleted_at;
        "#;

        let mut stmt = tx.prepare_cached(sql).map_err(DbError::with_sql(sql))?;

        let mut vec_out: Vec<(SchemaWort, Vec<SchemaWortGramType>)> =
            Vec::with_capacity(data.len());
        for d in data {
            let params: SqlWort = d.to_owned().into();
            let raw = stmt
                .query_one(
                    params_from_iter(params.insert_params()),
                    SchemaWort::from_sql,
                )
                .map_err(DbError::with_sql(sql))?;

            // This is neccesarry because Wort and GramType has a MxN table
            let vec_gram_type: Vec<_> = params
                .gram_type_ids
                .into_iter()
                .map(|d| InputWortGramType {
                    id_worte: raw.id,
                    id_gram_type: d,
                })
                .collect();

            let raw_gram = WortGramTypeRepo::bulk_insert_tx(tx, &vec_gram_type)?;

            vec_out.push((raw, raw_gram));
        }

        Ok(vec_out)
    }

    pub fn bulk_update(
        conn: &mut Connection,
        data: &[(i32, InputWort)],
    ) -> Result<Vec<(SchemaWort, Vec<SchemaWortGramType>)>, DbError> {
        let tx = conn.transaction()?;
        let out = Self::bulk_update_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    pub fn bulk_update_tx(
        tx: &Transaction,
        data: &[(i32, InputWort)],
    ) -> Result<Vec<(SchemaWort, Vec<SchemaWortGramType>)>, DbError> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            UPDATE worte SET 
                gender_id=?1,
                wort_de=?2,
                wort_es=?3,
                plural=?4,
                niveau_id=?5,
                example_de=?6,
                example_es=?7,
                verb_aux=?8,
                trennbar=?9,
                reflexiv=?10
            WHERE id=?11
            RETURNING 
                id, gender_id, wort_de, wort_es,
                plural, niveau_id, example_de, example_es, verb_aux,
                trennbar, reflexiv, created_at, deleted_at;
        "#;

        let mut stmt = tx.prepare(sql).map_err(DbError::with_sql(sql))?;
        let mut vec_out: Vec<(SchemaWort, Vec<SchemaWortGramType>)> =
            Vec::with_capacity(data.len());

        for (id, wort) in data {
            let params: SqlWort = wort.to_owned().into();

            let raw = stmt
                .query_row(
                    params_from_iter(params.update_params(id)),
                    SchemaWort::from_sql,
                )
                .map_err(DbError::with_sql(sql))?;

            // This is neccesary because Wort and GramType has a MxN table
            let vec_gram_type: Vec<InputWortGramType> = params
                .gram_type_ids
                .into_iter()
                .map(|d| InputWortGramType {
                    id_worte: raw.id,
                    id_gram_type: d,
                })
                .collect();

            WortGramTypeRepo::delete_by_wort_id_tx(tx, &[*id])?;
            let vec_wgt = WortGramTypeRepo::bulk_insert_tx(tx, &vec_gram_type)?;

            vec_out.push((raw, vec_wgt));
        }

        Ok(vec_out)
    }

    pub fn fetch_by_id(
        conn: &Connection,
        ids: &[i32],
    ) -> Result<Vec<(SchemaWort, Vec<SchemaWortGramType>)>, DbError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let placeholders = vec!["?"; ids.len()].join(",");

        let sql = format!(
            "
            SELECT 
                id, gender_id, wort_de, wort_es, plural, niveau_id, example_de,
                example_es, verb_aux, trennbar, reflexiv, created_at, deleted_at
            FROM worte w
            WHERE w.deleted_at is NULL AND
            w.id in ({placeholders})
            ORDER BY w.id;
        "
        );

        let mut stmt = conn.prepare(&sql).map_err(DbError::with_sql(&sql))?;

        let raw = stmt
            .query(params_from_iter(ids))
            .map_err(DbError::with_sql(&sql))?
            .mapped(SchemaWort::from_sql)
            .collect::<Result<Vec<SchemaWort>, _>>()
            .map_err(DbError::with_sql(&sql))?;

        let vec_out = Self::hydrate_gram_types(conn, raw)?;

        Ok(vec_out)
    }

    pub fn fetch_one(
        conn: &Connection,
        id: i32,
    ) -> Result<Option<(SchemaWort, Vec<SchemaWortGramType>)>, DbError> {
        let sql = format!(
            "
            SELECT 
                id, gender_id, wort_de, wort_es, plural, niveau_id, example_de,
                example_es, verb_aux, trennbar, reflexiv, created_at, deleted_at
            FROM worte w
            WHERE w.deleted_at is NULL AND w.id = ?1;
        "
        );

        let mut stmt = conn.prepare(&sql).map_err(DbError::with_sql(&sql))?;

        let raw = stmt
            .query_one(params![id], SchemaWort::from_sql)
            .optional()
            .map_err(DbError::with_sql(&sql))?;

        let raw = match raw {
            Some(v) => v,
            None => return Ok(None),
        };

        let mut vec_out = Self::hydrate_gram_types(conn, vec![raw])?;
        Ok(Some(vec_out.pop().unwrap()))
    }

    pub fn fetch_all_ids(
        conn: &Connection,
        limit: usize,
        last_id: i32,
    ) -> Result<Vec<i32>, DbError> {
        let sql = r#"
            SELECT id
            FROM worte w
            WHERE w.deleted_at is NULL AND id > ?1
            ORDER BY w.id
            LIMIT ?2;
        "#;

        let mut stmt = conn.prepare(sql).map_err(DbError::with_sql(sql))?;
        let vec_ids = stmt
            .query(params![last_id as i64, limit as i64])
            .map_err(DbError::with_sql(sql))?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<i32>, _>>()
            .map_err(DbError::with_sql(sql))?;

        Ok(vec_ids)
    }

    pub fn fetch_by_wort(
        conn: &Connection,
        worte: &[(String, String)],
    ) -> Result<Vec<(SchemaWort, Vec<SchemaWortGramType>)>, DbError> {
        if worte.is_empty() {
            return Ok(vec![]);
        }

        // COLLATE BINARY, just make a compare using bytes
        let sql = r#"
            SELECT 
                id, gender_id, wort_de, wort_es, plural, niveau_id, example_de,
                example_es, verb_aux, trennbar, reflexiv, created_at, deleted_at
            FROM worte w
            WHERE
                w.deleted_at is NULL
                AND wort_es = ?1 COLLATE BINARY
                AND wort_de = ?2 COLLATE BINARY
            ORDER BY w.id;
        "#;

        let mut vec_schema: Vec<SchemaWort> = vec![];
        for w in worte {
            let mut stmt = conn.prepare(&sql).map_err(DbError::with_sql(sql))?;

            let mut vec_raw = stmt
                .query(params![w.0, w.1])
                .map_err(DbError::with_sql(sql))?
                .mapped(SchemaWort::from_sql)
                .collect::<Result<Vec<_>, _>>()
                .map_err(DbError::with_sql(sql))?;

            vec_schema.append(&mut vec_raw);
        }

        let vec_out: Vec<(SchemaWort, Vec<SchemaWortGramType>)> =
            Self::hydrate_gram_types(conn, vec_schema)?;

        Ok(vec_out)
    }
}
