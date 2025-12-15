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

    pub fn fetch_id_neue_worte(conn: &Connection) -> Result<Vec<i32>> {
        let sql = "
            SELECT
                w.id
            FROM worte w
            WHERE NOT EXISTS (
                SELECT 1
                FROM worte_review wr
                WHERE wr.wort_id = w.id
            )
            AND w.deleted_at IS NULL
            ORDER BY w.id ASC;
        "
        .to_string();

        let mut stmt = conn.prepare_cached(&sql)?;

        let ids = stmt
            .query([])
            .context(format!("Sql - {}", sql))?
            .mapped(|r| r.get(0))
            .collect::<Result<Vec<i32>, _>>()?;

        Ok(ids)
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

    pub fn fetch_worte_without_audio(conn: &Connection) -> Result<Vec<Schema>> {
        let sql = "
            SELECT
                w.id,
                w.gender_id,
                w.wort_de,
                w.wort_es,
                w.plural,
                w.niveau_id,
                w.example_de,
                w.example_es,
                w.verb_aux,
                w.trennbar,
                w.reflexiv,
                w.created_at,
                w.deleted_at
            FROM worte w
            LEFT JOIN worte_audio wa ON w.id = wa.wort_id 
            WHERE w.deleted_at IS NULL AND wa.wort_id is NULL
            ORDER BY w.id ASC;
        "
        .to_string();

        let mut stmt = conn.prepare_cached(&sql)?;

        let raws = stmt
            .query([])
            .context(format!("Sql - {}", sql))?
            .mapped(Raw::from_sql)
            .collect::<Result<Vec<Raw>, _>>()?;

        let vec_out = Schema::from_vec_raw(raws)?;
        Ok(vec_out)
    }
}
