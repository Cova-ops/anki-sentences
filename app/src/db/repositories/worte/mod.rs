use color_eyre::eyre::{Context, Result};
use rusqlite::{Connection, Transaction, params};
use sql_model::{FromRaw, SqlNew, SqlRaw};

use crate::db::{
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
    pub fn bulk_insert(conn: &mut Connection, data: &[New]) -> Result<Vec<Schema>> {
        let tx = conn.transaction()?;
        let out = Self::bulk_insert_tx(&tx, data)?;
        println!("out: {:#?}", out);
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

        println!("vec_out: {:#?}", vec_out);
        Ok(vec_out)
    }
}
