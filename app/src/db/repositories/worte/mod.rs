use std::collections::HashMap;

use color_eyre::eyre::{Context, Result};
use rusqlite::{Connection, Transaction, params, params_from_iter};
use sql_model::{FromRaw, SqlNew, SqlRaw};

use crate::db::{
    schemas::{
        gram_type::GramTypeSchema,
        worte::{NewWorteSchema as New, RawWorteSchema as Raw, WorteSchema as Schema},
        worte_gram_type::{NewWorteGramTypeSchema, WorteGramTypeSchema},
    },
    worte_gram_type::WorteGramTypeRepo,
};

#[cfg(test)]
mod worte_test;

pub struct WorteRepo;

impl WorteRepo {
    pub fn bulk_insert(conn: &mut Connection, data: &[New]) -> Result<Vec<Schema>> {
        let tx = conn.transaction()?;
        let out = Self::bulk_insert_tx(&tx, data)?;
        // println!("out: {:#?}", out);
        tx.commit()?;
        Ok(out)
    }

    pub fn bulk_insert_tx(tx: &Transaction, data: &[New]) -> Result<Vec<Schema>> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            INSERT INTO 
                worte (gender_id,wort_de,wort_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv)
            VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
            RETURNING id,gender_id,wort_de,wort_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv,created_at,deleted_at;
        "#;

        let mut stmt = tx.prepare_cached(sql)?;

        let mut vec_out = Vec::with_capacity(data.len());
        for d in data {
            let raw = stmt
                .query_one(d.to_params(), Raw::from_sql)
                .context(format!("sql: {}, params: {:#?}", sql, d))?;
            vec_out.push(Schema::from_raw(raw)?);
        }

        let mut vec_mn: Vec<NewWorteGramTypeSchema> = vec![];
        for (wort, new) in vec_out.iter_mut().zip(data.iter()) {
            for gram_type_id in &new.gram_type {
                // Llenamos arreglo para la tabla NxM
                vec_mn.push(NewWorteGramTypeSchema {
                    id_worte: wort.id,
                    id_gram_type: *gram_type_id,
                });

                // Llenamos arreglo para la información del Schema para el regreso
                wort.gram_type_id
                    .push(GramTypeSchema::from_id(*gram_type_id)?);
            }
        }

        WorteGramTypeRepo::bulk_insert_tx(tx, &vec_mn)?;

        Ok(vec_out)
    }

    pub fn bulk_update(conn: &mut Connection, data: &[(i32, New)]) -> Result<Vec<Schema>> {
        let tx = conn.transaction()?;
        let out = Self::bulk_update_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    pub fn bulk_update_tx(tx: &Transaction, data: &[(i32, New)]) -> Result<Vec<Schema>> {
        if data.is_empty() {
            return Ok(vec![]);
        }

        let sql = r#"
            UPDATE worte SET 
                gender_id=?2,
                wort_de=?3,
                wort_es=?4,
                plural=?5,
                niveau_id=?6,
                example_de=?7,
                example_es=?8,
                verb_aux=?9,
                trennbar=?10,
                reflexiv=?11
            WHERE id=?1
            RETURNING 
                id, gender_id, wort_de, wort_es,
                plural, niveau_id, example_de, example_es, verb_aux,
                trennbar, reflexiv, created_at, deleted_at;
        "#;

        let mut stmt = tx.prepare_cached(sql)?;

        let mut vec_out = Vec::with_capacity(data.len());
        for d in data {
            let params = params![
                d.0,
                d.1.gender_id,
                d.1.worte_de,
                d.1.worte_es,
                d.1.plural,
                d.1.niveau_id,
                d.1.example_de,
                d.1.example_es,
                d.1.verb_aux,
                d.1.trennbar,
                d.1.reflexiv
            ];

            let raw = stmt
                .query_one(params, Raw::from_sql)
                .context(format!("sql: {}, params: {:#?}", sql, d))?;
            vec_out.push(Schema::from_raw(raw)?);
        }

        let mut vec_mn: Vec<NewWorteGramTypeSchema> = vec![];
        for (wort, new) in vec_out.iter_mut().zip(data.iter()) {
            for gram_type_id in &new.1.gram_type {
                // Llenamos arreglo para la tabla NxM
                vec_mn.push(NewWorteGramTypeSchema {
                    id_worte: wort.id,
                    id_gram_type: *gram_type_id,
                });

                // Llenamos arreglo para la información del Schema para el regreso
                wort.gram_type_id
                    .push(GramTypeSchema::from_id(*gram_type_id)?);
            }
        }

        WorteGramTypeRepo::bulk_insert_tx(tx, &vec_mn)?;

        Ok(vec_out)
    }

    pub fn fetch_by_id(conn: &Connection, ids: &[i32]) -> Result<Vec<Schema>> {
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

        let mut stmt = conn.prepare(&sql)?;
        let raw = stmt
            .query(params_from_iter(ids.iter()))?
            .mapped(Raw::from_sql)
            .collect::<Result<Vec<Raw>, _>>()?;

        // Tenemos que consultar de la tabla WorteGramType cuales le corresponde a cada palabra
        let worte_gram_type = WorteGramTypeRepo::fetch_by_wort_id(conn, ids)?;
        let hash_worte_gram_type = {
            let mut hash: HashMap<i32, Vec<WorteGramTypeSchema>> = HashMap::new();

            for wgt in worte_gram_type {
                hash.entry(wgt.id_worte).or_default().push(wgt);
            }

            hash
        };

        let mut vec_out = Schema::from_vec_raw(raw)?;

        // Llenamos el schema de Worte con la info de GramType
        for wort in vec_out.iter_mut() {
            if let Some(vec_wgt) = hash_worte_gram_type.get(&wort.id) {
                for wgt in vec_wgt {
                    // Llenamos arreglo para la información del Schema para el regreso
                    wort.gram_type_id
                        .push(GramTypeSchema::from_id(wgt.id_gram_type)?);
                }
            };
        }

        Ok(vec_out)
    }

    pub fn fetch_all_ids(conn: &Connection, limit: usize, last_id: i32) -> Result<Vec<i32>> {
        let sql = r#"
            SELECT id
            FROM worte w
            WHERE w.deleted_at is NULL AND id > ?1
            ORDER BY w.id
            LIMIT ?2;
        "#;

        let mut stmt = conn.prepare(&sql)?;
        let vec_ids = stmt
            .query(params![last_id as i64, limit as i64])?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(vec_ids)
    }

    pub fn fetch_by_wort(conn: &Connection, worte: &[(String, String)]) -> Result<Vec<Schema>> {
        if worte.is_empty() {
            return Ok(vec![]);
        }

        let mut vec_out: Vec<Schema> = vec![];
        for w in worte {
            let sql = format!(
                r#"
                    SELECT 
                        id, gender_id, wort_de, wort_es, plural, niveau_id, example_de,
                        example_es, verb_aux, trennbar, reflexiv, created_at, deleted_at
                    FROM worte w
                    WHERE
                        w.deleted_at is NULL
                        AND wort_es = ?1 COLLATE BINARY
                        AND wort_de = ?2 COLLATE BINARY
                    ORDER BY w.id;
                "#
            );

            let mut stmt = conn.prepare(&sql)?;
            let vec_raw = stmt
                .query(params![w.0, w.1])?
                .mapped(Raw::from_sql)
                .collect::<Result<Vec<Raw>, _>>()?;

            let mut vec_worte = Schema::from_vec_raw(vec_raw)?;
            vec_out.append(&mut vec_worte);
        }

        Ok(vec_out)
    }
}
