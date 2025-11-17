use color_eyre::eyre::{Context, Result};
use rusqlite::params;

use crate::{
    ctx,
    db::{
        get_conn,
        schemas::geschichtlich_setze::{GeschichtlichSetzeSchema, NewGeschichtlichSetzeSchema},
    },
    helpers, to_strings, with_ctx,
};

#[derive(Debug)]
struct Raw {
    id: i32,
    setze_id: i32,
    result: i32,
    created_at: String,
    deleted_at: Option<String>,
}

pub fn insert_db(
    data: &[NewGeschichtlichSetzeSchema],
    result: bool,
) -> Result<Vec<GeschichtlichSetzeSchema>> {
    let mut conn = get_conn();
    let tx = conn.transaction().context(ctx!())?;

    let sql = r#"
        INSERT INTO geschichtlich_setze (setze_id, result)
        VALUES (?1,?2)
        RETURNING id,setze_id, result, created_at,deleted_at;"#;

    let mut vec_raw: Vec<Raw> = Vec::with_capacity(data.len());

    let mut stmt = tx
        .prepare_cached(sql)
        .context(with_ctx!(format!("sql: {}", sql)))?;
    for d in data {
        let raw: Raw = stmt
            .query_row(params![d.setze_id, result], |r| {
                Ok(Raw {
                    id: r.get(0)?,
                    setze_id: r.get(1)?,
                    result: r.get(2)?,
                    created_at: r.get(3)?,
                    deleted_at: r.get(4).ok(),
                })
            })
            .context(with_ctx!(format!(
                "sql: {} & params: {:#?}",
                sql,
                to_strings!(d.setze_id, result)
            )))?;
        vec_raw.push(raw);
    }

    drop(stmt);
    tx.commit().context(ctx!())?;

    let vec_result: Vec<GeschichtlichSetzeSchema> = vec_raw
        .into_iter()
        .map(|r| {
            let created_at = helpers::time::string_2_datetime(Some(r.created_at)).unwrap();
            let deleted_at = helpers::time::string_2_datetime(r.deleted_at);

            GeschichtlichSetzeSchema {
                id: r.id,
                setze_id: r.setze_id,
                result: r.result != 0,
                created_at,
                deleted_at,
            }
        })
        .collect();

    Ok(vec_result)
}
