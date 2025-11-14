use color_eyre::eyre::{Context, Result};

use crate::{ctx, db::get_conn};

pub mod geschichtlich_setze;
pub mod schwirigkeit_liste;
pub mod setze;
pub mod wort;

pub fn init_schemas() -> Result<()> {
    let conn = get_conn();
    // Activar las llaves foráneas
    conn.execute("PRAGMA foreign_keys = ON", [])
        .context(ctx!())?;

    // Dificultad
    conn.execute(schwirigkeit_liste::CREATE_STR_TABLE_SCHWIRIGKEIT_LISTE, [])
        .context(ctx!())?;

    // Oraciones
    conn.execute(setze::CREATE_STR_TABLE_SETZE, [])
        .context(ctx!())?;
    conn.execute_batch(setze::CREATE_STR_INDEX_SETZE)
        .context(ctx!())?;

    // Historico de oraciones
    conn.execute(
        geschichtlich_setze::CREATE_STR_TABLE_GESCHICHTLICH_SETZE,
        [],
    )
    .context(ctx!())?;
    conn.execute_batch(geschichtlich_setze::CREATE_STR_INDEX_GESCHICHTLICH_SETZE)
        .context(ctx!())?;

    // Palabras
    conn.execute(wort::CREATE_STR_TABLE_WORTE, [])
        .context(ctx!())?;
    conn.execute_batch(wort::CREATE_STR_INDEX_WORTE)
        .context(ctx!())?;

    Ok(())
}
