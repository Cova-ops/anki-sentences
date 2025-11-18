use color_eyre::eyre::{Context, Result};
use rusqlite::params;

use crate::{
    ctx,
    db::{
        get_conn,
        schemas::geschichtlich_setze::{
            GeschichtlichSetzeSchema, NewGeschichtlichSetzeSchema, RawGeschichtlichSetzeSchema,
        },
        traits::FromRaw,
    },
    with_ctx,
};

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

    let mut vec_raw: Vec<RawGeschichtlichSetzeSchema> = Vec::with_capacity(data.len());

    let mut stmt = tx
        .prepare_cached(sql)
        .context(with_ctx!(format!("sql: {}", sql)))?;
    for d in data {
        let raw: RawGeschichtlichSetzeSchema = stmt
            .query_row(params![d.setze_id, result], |r| {
                Ok(RawGeschichtlichSetzeSchema {
                    id: r.get(0)?,
                    setze_id: r.get(1)?,
                    result: r.get(2)?,
                    created_at: r.get(3)?,
                    deleted_at: r.get(4)?,
                })
            })
            .context(with_ctx!(format!("sql: {} & params: {:#?}", sql, d)))?;
        vec_raw.push(raw);
    }

    drop(stmt);
    tx.commit().context(ctx!())?;

    let vec_result = GeschichtlichSetzeSchema::from_vec_raw(vec_raw)?;
    Ok(vec_result)
}
