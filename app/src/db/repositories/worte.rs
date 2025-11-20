use color_eyre::eyre::Result;
use rusqlite::{Connection, Transaction, params};
use sql_model::{FromRaw, SqlRaw};

use crate::db::{
    schemas::{
        worte::{NewWorteSchema as New, RawWorteSchema as Raw, WorteSchema as Schema},
        worte_gram_type::NewWorteGramTypeSchema,
    },
    worte_gram_type::WorteGramTypeRepo,
};

pub struct WorteRepo;

impl WorteRepo {
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
            INSERT INTO 
                wort (gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv)
            VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
            RETURNING id,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv,created_at,deleted_at;
        "#;

        let mut stmt = tx.prepare_cached(sql)?;

        let mut vec_out = Vec::with_capacity(data.len());
        for d in data {
            let params = params![
                d.gender_id,
                d.worte_de,
                d.worte_es,
                d.plural,
                d.niveau_id,
                d.example_de,
                d.example_es,
                d.verb_aux,
                d.trennbar,
                d.reflexiv
            ];
            let raw = stmt.query_one(params, Raw::from_sql)?;
            vec_out.push(Schema::from_raw(raw)?);
        }

        let mut vec_mn: Vec<NewWorteGramTypeSchema> = vec![];
        for (wort, new) in vec_out.iter().zip(data.iter()) {
            for gram_type_id in &new.gram_type {
                vec_mn.push(NewWorteGramTypeSchema {
                    id_worte: wort.id,
                    id_gram_type: *gram_type_id,
                });
            }
        }

        WorteGramTypeRepo::bulk_insert_tx(&tx, &vec_mn)?;
        Ok(vec_out)
    }
}
