use color_eyre::eyre::Result;
use rusqlite::params;

use crate::db::{
    get_conn,
    schemas::niveau_worte::{NewNiveauWorteSchema, NiveauWorteSchema, RawNiveauWorteSchema},
    traits::FromRaw,
};

pub fn bulk_insert(data: &[NewNiveauWorteSchema]) -> Result<Vec<NiveauWorteSchema>> {
    let sql = r#"
        INSERT INTO niveau_worte (id, niveau)
        VALUES (?1, ?2) ON CONFLICT(id) DO UPDATE SET niveau = ?2
        RETURNING id, niveau,created_at,deleted_at;
    "#;

    let mut conn = get_conn();
    let tx = conn.transaction()?;

    let mut vec_raw: Vec<RawNiveauWorteSchema> = vec![];
    let mut stmt = tx.prepare_cached(sql)?;

    for d in data {
        let raw = stmt.query_one(params![d.id, d.niveau], |r| {
            Ok(RawNiveauWorteSchema {
                id: r.get(0)?,
                niveau: r.get(1)?,
                created_at: r.get(2)?,
                deleted_at: r.get(3).ok(),
            })
        })?;

        vec_raw.push(raw)
    }

    drop(stmt);
    tx.commit()?;

    let vec_result = NiveauWorteSchema::from_vec_raw(vec_raw)?;

    Ok(vec_result)
}
