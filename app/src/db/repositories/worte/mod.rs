use std::collections::HashMap;

use color_eyre::eyre::{Context, Result};
use rusqlite::{Connection, Transaction, params, params_from_iter};
use sql_model::{FromRaw, SqlNew, SqlRaw};

use crate::db::{
    gram_type::GramTypeRepo,
    schemas::{
        gram_type::GramTypeSchema,
        worte::{NewWorteSchema as New, RawWorteSchema as Raw, WorteSchema as Schema},
        worte_gram_type::NewWorteGramTypeSchema,
    },
    worte_gram_type::WorteGramTypeRepo,
};

#[cfg(test)]
mod worte_test;

pub struct WorteRepo;

impl WorteRepo {
    fn hydrate_gram_types(conn: &Connection, worte: &mut [Schema]) -> Result<()> {
        if worte.is_empty() {
            return Ok(());
        }

        let ids: Vec<i32> = worte.iter().map(|w| w.id).collect();

        let worte_gram_type = WorteGramTypeRepo::fetch_by_wort_id(conn, &ids)?;

        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for wgt in worte_gram_type {
            map.entry(wgt.id_worte).or_default().push(wgt.id_gram_type);
        }

        for w in worte.iter_mut() {
            if let Some(gt_ids) = map.get(&w.id) {
                // si NO quieres cache estático, aquí en vez de from_id()
                // harías un fetch a gram_type por ids y lo mapearías.
                w.gram_type_id = gt_ids
                    .iter()
                    .map(|id| GramTypeSchema::from_id(*id)) // o fetch_by_ids(...)
                    .collect::<Result<Vec<_>>>()?;
            }
        }

        Ok(())
    }

    #[cfg_attr(feature = "tested", doc = "v0.2")]
    pub fn bulk_insert(conn: &mut Connection, data: &[New]) -> Result<Vec<Schema>> {
        let tx = conn.transaction()?;
        let out = Self::bulk_insert_tx(&tx, data)?;
        // println!("out: {:#?}", out);
        tx.commit()?;
        Ok(out)
    }

    #[cfg_attr(feature = "tested", doc = "v0.2")]
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

    #[cfg_attr(feature = "tested", doc = "v0.2")]
    pub fn bulk_update(conn: &mut Connection, data: &[(i32, New)]) -> Result<Vec<Schema>> {
        let tx = conn.transaction()?;
        let out = Self::bulk_update_tx(&tx, data)?;
        tx.commit()?;
        Ok(out)
    }

    #[cfg_attr(feature = "tested", doc = "v0.2")]
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

        let mut stmt = tx.prepare(sql)?;

        let mut vec_out: Vec<Schema> = Vec::with_capacity(data.len());
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
                .query_row(params, Raw::from_sql)
                .with_context(|| format!("sql: {sql}\nupdate_id: {}\nnew_data: {:#?}", d.0, d.1))?;

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

        let vec_remove_id_worte: Vec<i32> = vec_mn.iter().map(|x| x.id_worte).collect();
        WorteGramTypeRepo::delete_by_wort_id_tx(tx, &vec_remove_id_worte)?;
        WorteGramTypeRepo::bulk_insert_tx(tx, &vec_mn)?;

        Ok(vec_out)
    }

    #[cfg_attr(feature = "tested", doc = "v0.2")]
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

        let mut vec_out = Schema::from_vec_raw(raw)?;
        Self::hydrate_gram_types(conn, &mut vec_out)?;

        Ok(vec_out)
    }

    #[cfg_attr(feature = "tested", doc = "v0.2")]
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

    #[cfg_attr(feature = "tested", doc = "v0.2")]
    pub fn fetch_by_wort(conn: &Connection, worte: &[(String, String)]) -> Result<Vec<Schema>> {
        if worte.is_empty() {
            return Ok(vec![]);
        }

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

        let mut vec_out: Vec<Schema> = vec![];
        for w in worte {
            let mut stmt = conn.prepare(&sql)?;
            let vec_raw = stmt
                .query(params![w.0, w.1])?
                .mapped(Raw::from_sql)
                .collect::<Result<Vec<Raw>, _>>()?;

            let mut vec_schema = Schema::from_vec_raw(vec_raw)?;
            vec_out.append(&mut vec_schema);
        }

        Self::hydrate_gram_types(conn, &mut vec_out)?;

        Ok(vec_out)
    }
}
