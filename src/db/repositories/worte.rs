use color_eyre::eyre::Result;
use rusqlite::params;

use crate::db::{
    WorteGramTypeRepo, WorteRepo, get_conn,
    schemas::{
        worte::{NewWorteSchema, RawWorteSchema, WorteSchema},
        worte_gram_type::NewWorteGramTypeSchema,
    },
    traits::FromRaw,
};

pub fn bulk_insert(data: &[NewWorteSchema]) -> Result<Vec<WorteSchema>> {
    if data.is_empty() {
        return Ok(vec![]);
    }

    let sql = r#"
    INSERT INTO 
        wort (gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv)
    VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
    RETURNING id,gender_id,worte_de,worte_es,plural,niveau_id,example_de,example_es,verb_aux,trennbar,reflexiv,created_at,deleted_at;
    "#;

    let mut conn = get_conn();
    let tx = conn.transaction()?;
    let mut stmt = tx.prepare_cached(sql)?;

    let mut vec_raw: Vec<RawWorteSchema> = Vec::with_capacity(data.len());
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
        let raw = stmt.query_one(params, |r| {
            Ok(RawWorteSchema {
                id: r.get(0)?,
                gender_id: r.get(1)?,
                worte_de: r.get(2)?,
                worte_es: r.get(3)?,
                plural: r.get(4)?,
                niveau_id: r.get(5)?,
                example_de: r.get(6)?,
                example_es: r.get(7)?,
                verb_aux: r.get(8)?,
                trennbar: r.get(9)?,
                reflexiv: r.get(10)?,
                created_at: r.get(11)?,
                deleted_at: r.get(12)?,
            })
        })?;
        vec_raw.push(raw);
    }

    drop(stmt);

    let mut vec_mn: Vec<NewWorteGramTypeSchema> = vec![];
    for (wort, new) in vec_raw.iter().zip(data.iter()) {
        for gram_type_id in &new.gram_type {
            vec_mn.push(NewWorteGramTypeSchema {
                id_worte: wort.id,
                id_gram_type: *gram_type_id,
            });
        }
    }

    WorteGramTypeRepo::bulk_insert_tx(&tx, &vec_mn)?;

    tx.commit()?;

    let vec_result = WorteSchema::from_vec_raw(vec_raw)?;
    Ok(vec_result)
}
