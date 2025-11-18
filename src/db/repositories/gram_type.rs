use color_eyre::eyre::Result;
use rusqlite::params;

use crate::db::{
    get_conn,
    schemas::gram_type::{GramTypeSchema, NewGramTypeSchema, RawGramTypeSchema},
    traits::FromRaw,
};

pub fn bulk_insert(data: &[NewGramTypeSchema]) -> Result<Vec<GramTypeSchema>> {
    let sql = r#"
    INSERT INTO gram_type (id, code, name)
        VALUES (?1,?2,?3) 
        ON CONFLICT(id) DO UPDATE SET code = ?2, name = ?3
        ON CONFLICT(code) DO UPDATE SET name = ?3
        RETURNING id,code,name,created_at,deleted_at;
    "#;

    let mut conn = get_conn();
    let tx = conn.transaction()?;
    let mut stmt = tx.prepare_cached(sql)?;

    let mut vec_raw: Vec<RawGramTypeSchema> = Vec::with_capacity(data.len());
    for d in data {
        let raw = stmt.query_one(params![d.id, d.code, d.name], |r| {
            Ok(RawGramTypeSchema {
                id: r.get(0)?,
                code: r.get(1)?,
                name: r.get(2)?,
                created_at: r.get(3)?,
                deleted_at: r.get(4).ok(),
            })
        })?;

        vec_raw.push(raw);
    }

    drop(stmt);
    tx.commit()?;

    let vec_result = GramTypeSchema::from_vec_raw(vec_raw)?;
    Ok(vec_result)
}
