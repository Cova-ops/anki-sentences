use color_eyre::eyre::{Context, Result};
use rusqlite::params;

use crate::{
    ctx,
    db::{get_conn, schemas::gender_worte::NewGenderWorteSchema},
    with_ctx,
};

pub fn bulk_insert(data: &[NewGenderWorteSchema]) -> Result<()> {
    let sql = r#"
    INSERT INTO gender_worte (id, gender, artikel)
    VALUES (?1, ?2, ?3) ON CONFLICT(id) DO UPDATE SET gender = ?2, artikel = ?3;
    "#;
    let mut conn = get_conn();
    let tx = conn.transaction().context(ctx!())?;

    {
        let mut stmt = tx
            .prepare_cached(sql)
            .context(with_ctx!(format!("error sql: {}", sql)))?;
        for d in data {
            stmt.execute(params![d.id, d.gender, d.artikel])
                .context(with_ctx!(format!("error sql: {}, params: {:#?}", sql, d)))?;
        }
    }

    tx.commit().context(ctx!())?;

    Ok(())
}
