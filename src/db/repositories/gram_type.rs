use color_eyre::eyre::{Context, Result};
use rusqlite::params;

use crate::{
    ctx,
    db::{
        get_conn,
        schemas::gram_type::{GramTypeSchema, NewGramTypeSchema},
    },
    helpers, with_ctx,
};

struct Raw {
    id: i32,
    code: String,
    name: String,
    created_at: String,
    deleted_at: Option<String>,
}

pub fn bulk_insert(data: &[NewGramTypeSchema]) -> Result<Vec<GramTypeSchema>> {
    let sql = r#"
        INSERT INTO gram_type (id, code, name)
        VALUES (?1,?2,?3) 
        ON CONFLICT(id) DO UPDATE SET code = ?2, name = ?3
        ON CONFLICT(code) DO UPDATE SET name = ?3
        RETURNING id,code,name,created_at,deleted_at;
    "#;

    let mut conn = get_conn();
    let tx = conn.transaction().context(ctx!())?;

    let mut stmt = tx
        .prepare_cached(sql)
        .context(with_ctx!(format!("error sql: {}", sql)))?;

    let mut vec_raw: Vec<Raw> = Vec::with_capacity(data.len());
    for d in data {
        let raw: Raw = stmt
            .query_one(params![d.id, d.code, d.name], |r| {
                Ok(Raw {
                    id: r.get(0)?,
                    code: r.get(1)?,
                    name: r.get(2)?,
                    created_at: r.get(3)?,
                    deleted_at: r.get(4).ok(),
                })
            })
            .context(ctx!())?;

        vec_raw.push(raw);
    }

    drop(stmt);
    tx.commit().context(ctx!())?;

    let vec_result: Vec<GramTypeSchema> = vec_raw
        .into_iter()
        .map(|r| {
            let created_at = helpers::time::string_2_datetime(Some(r.created_at)).unwrap();
            let deleted_at = helpers::time::string_2_datetime(r.deleted_at);

            GramTypeSchema {
                id: r.id,
                code: r.code,
                name: r.name,
                created_at,
                deleted_at,
            }
        })
        .collect();

    Ok(vec_result)
}
