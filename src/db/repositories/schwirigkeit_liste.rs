use color_eyre::eyre::Result;
use rusqlite::params;

use crate::db::{
    get_conn,
    schemas::schwirigkeit_liste::{
        NewSchwirigkeitListeSchema, RawSchwirigkeitListeSchema, SchwirigkeitListeSchema,
    },
    traits::FromRaw,
};

pub fn bulk_insert(data: &[NewSchwirigkeitListeSchema]) -> Result<Vec<SchwirigkeitListeSchema>> {
    let sql = "INSERT INTO schwirigkeit_liste (id, schwirigkeit)
        VALUES (?1,?2) ON CONFLICT(id) DO UPDATE SET schwirigkeit = ?3
        RETURNING id,schwirigkeit,created_at,deleted_at;";

    let mut conn = get_conn();
    let tx = conn.transaction()?;

    let mut vec_raw: Vec<RawSchwirigkeitListeSchema> = vec![];
    let mut stmt = tx.prepare_cached(sql)?;

    for d in data {
        let raw = stmt.query_one(params![d.id, d.schwirigkeit, d.schwirigkeit], |r| {
            Ok(RawSchwirigkeitListeSchema {
                id: r.get(0)?,
                schwirigkeit: r.get(1)?,
                created_at: r.get(2)?,
                deleted_at: r.get(3)?,
            })
        })?;
        vec_raw.push(raw);
    }

    drop(stmt);
    tx.commit()?;

    let vec_result = SchwirigkeitListeSchema::from_vec_raw(vec_raw)?;

    Ok(vec_result)
}
