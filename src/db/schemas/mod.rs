use color_eyre::eyre::{Context, Result};

use crate::{ctx, db::get_conn};

pub mod gender_worte;
pub mod geschichtlich_setze;
pub mod gram_type;
pub mod niveau_worte;
pub mod schwirigkeit_liste;
pub mod setze;
pub mod wort;
pub mod wort_gram_type;

pub fn init_schemas() -> Result<()> {
    let conn = get_conn();
    // Activar las llaves foráneas
    conn.execute("PRAGMA foreign_keys = ON", [])
        .context(ctx!())?;

    // Dificultad
    conn.execute(schwirigkeit_liste::CREATE_STR_TABLE_SCHWIRIGKEIT_LISTE, [])
        .context(ctx!())?;

    // Gender Worte
    conn.execute(gender_worte::CREATE_STR_TABLE_GENDER_WORTE, [])
        .context(ctx!())?;
    conn.execute_batch(gender_worte::CREATE_STR_INDEX_GENDER_WORTE)
        .context(ctx!())?;

    // Niveau worte
    conn.execute(niveau_worte::CREATE_STR_TABLE_GENDER_WORTE, [])
        .context(ctx!())?;
    conn.execute_batch(niveau_worte::CREATE_STR_INDEX_NIVEAU_WORTE)
        .context(ctx!())?;

    // Gram Type
    conn.execute(gram_type::CREATE_STR_TABLE_GRAM_TYPE, [])
        .context(ctx!())?;
    conn.execute_batch(gram_type::CREATE_STR_INDEX_GRAM_TYPE)
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
