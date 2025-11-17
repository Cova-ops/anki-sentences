use color_eyre::eyre::{Context, Result};
use rusqlite::params;

use crate::{
    ctx,
    db::{
        get_conn,
        schemas::gender_worte::{GenderWorteSchema, NewGenderWorteSchema},
    },
    helpers, with_ctx,
};

#[derive(Debug)]
struct Raw {
    id: i32,
    gender: String,
    artikel: String,
    created_at: String,
    deleted_at: Option<String>,
}

pub fn bulk_insert(data: &[NewGenderWorteSchema]) -> Result<Vec<GenderWorteSchema>> {
    let sql = r#"
    INSERT INTO gender_worte (id, gender, artikel)
    VALUES (?1, ?2, ?3) ON CONFLICT(id) DO UPDATE SET gender = ?2, artikel = ?3
    RETURNING id,gender,artikel,created_at,deleted_at;
    "#;
    let mut conn = get_conn();
    let tx = conn.transaction().context(ctx!())?;

    let mut vec_raw: Vec<Raw> = vec![];
    {
        let mut stmt = tx
            .prepare_cached(sql)
            .context(with_ctx!(format!("error sql: {}", sql)))?;
        for d in data {
            let raw = stmt
                .query_one(params![d.id, d.gender, d.artikel], |r| {
                    Ok(Raw {
                        id: r.get(0)?,
                        gender: r.get(1)?,
                        artikel: r.get(2)?,
                        created_at: r.get(3)?,
                        deleted_at: r.get(4)?,
                    })
                })
                .context(with_ctx!(format!("error sql: {}, params: {:#?}", sql, d)))?;

            vec_raw.push(raw);
        }
    }

    tx.commit().context(ctx!())?;

    let vec_result = vec_raw
        .into_iter()
        .map(|r| {
            let created_at = helpers::time::string_2_datetime(Some(r.created_at)).unwrap();
            let deleted_at = helpers::time::string_2_datetime(r.deleted_at);

            GenderWorteSchema {
                id: r.id,
                gender: r.gender,
                artikel: r.artikel,
                created_at,
                deleted_at,
            }
        })
        .collect();

    Ok(vec_result)
}
