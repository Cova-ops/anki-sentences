use color_eyre::eyre::{Context, Result};
use rusqlite::params;

use crate::{
    ctx,
    db::{
        get_conn,
        schemas::schwirigkeit_liste::{NewSchwirigkeitListeSchema, SchwirigkeitListeSchema},
    },
    helpers, with_ctx,
};

struct Raw {
    id: i32,
    schwirigkeit: String,
    created_at: String,
    deleted_at: Option<String>,
}

pub fn bulk_insert(data: &[NewSchwirigkeitListeSchema]) -> Result<Vec<SchwirigkeitListeSchema>> {
    let sql = "INSERT INTO schwirigkeit_liste (id, schwirigkeit)
        VALUES (?1,?2) ON CONFLICT(id) DO UPDATE SET schwirigkeit = ?3
        RETURNING id,schwirigkeit,created_at,deleted_at;";

    let mut conn = get_conn();
    let tx = conn.transaction().context(ctx!())?;

    let mut vec_raw: Vec<Raw> = vec![];
    {
        let mut stmt = tx
            .prepare_cached(sql)
            .context(with_ctx!(format!("error sql: {}", sql)))?;

        for d in data {
            let raw: Raw = stmt
                .query_one(params![d.id, d.schwirigkeit, d.schwirigkeit], |r| {
                    Ok(Raw {
                        id: r.get(0)?,
                        schwirigkeit: r.get(1)?,
                        created_at: r.get(2)?,
                        deleted_at: r.get(3)?,
                    })
                })
                .context(with_ctx!(format!("sql: {} & params: {:#?}", sql, d)))?;
            vec_raw.push(raw);
        }
    }

    tx.commit().context(ctx!())?;

    let vec_result = vec_raw
        .into_iter()
        .map(|r| {
            let created_at = helpers::time::string_2_datetime(Some(r.created_at)).unwrap();
            let deleted_at = helpers::time::string_2_datetime(r.deleted_at);

            SchwirigkeitListeSchema {
                id: r.id,
                schwirigkeit: r.schwirigkeit,
                created_at,
                deleted_at,
            }
        })
        .collect();

    Ok(vec_result)
}
