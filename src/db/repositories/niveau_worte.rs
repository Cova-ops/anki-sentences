use color_eyre::eyre::{Context, Result};
use rusqlite::params;

use crate::{
    ctx,
    db::{get_conn, schemas::niveau_worte::NewNiveauWorteSchema},
    with_ctx,
};

pub fn bulk_insert(data: &[NewNiveauWorteSchema]) -> Result<()> {
    let sql = r#"
    INSERT INTO niveau_worte (id, niveau)
    VALUES (?1, ?2) ON CONFLICT(id) DO UPDATE SET niveau = ?2;
    "#;

    let mut conn = get_conn();
    let tx = conn.transaction().context(ctx!())?;

    {
        let mut stmt = tx
            .prepare_cached(sql)
            .context(with_ctx!(format!("error sql: {}", sql)))?;

        for d in data {
            stmt.execute(params![d.id, d.niveau])
                .context(with_ctx!(format!("error sql: {}. params: {:#?}", sql, d)))?;
        }
    }

    tx.commit().context(ctx!())?;

    Ok(())
}
