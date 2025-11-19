use color_eyre::eyre::Result;

use crate::db::get_conn;

pub mod gender_worte;
pub mod geschichtlich_setze;
pub mod gram_type;
pub mod niveau_worte;
pub mod schwirigkeit_liste;
pub mod setze;
pub mod worte;
pub mod worte_gram_type;

pub fn init_schemas() -> Result<()> {
    let conn = get_conn();
    // Activar las llaves foráneas
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    // Dificultad
    conn.execute(schwirigkeit_liste::CREATE_STR_TABLE_SCHWIRIGKEIT_LISTE, [])?;

    // Gender Worte
    conn.execute(gender_worte::CREATE_STR_TABLE_GENDER_WORTE, [])?;
    conn.execute_batch(gender_worte::CREATE_STR_INDEX_GENDER_WORTE)?;

    // Niveau worte
    conn.execute(niveau_worte::CREATE_STR_TABLE_GENDER_WORTE, [])?;
    conn.execute_batch(niveau_worte::CREATE_STR_INDEX_NIVEAU_WORTE)?;

    // Gram Type
    conn.execute(gram_type::CREATE_STR_TABLE_GRAM_TYPE, [])?;
    conn.execute_batch(gram_type::CREATE_STR_INDEX_GRAM_TYPE)?;

    // Oraciones
    conn.execute(setze::CREATE_STR_TABLE_SETZE, [])?;
    conn.execute_batch(setze::CREATE_STR_INDEX_SETZE)?;

    // Historico de oraciones
    conn.execute(
        geschichtlich_setze::CREATE_STR_TABLE_GESCHICHTLICH_SETZE,
        [],
    )?;
    conn.execute_batch(geschichtlich_setze::CREATE_STR_INDEX_GESCHICHTLICH_SETZE)?;

    // Palabras
    conn.execute(worte::CREATE_STR_TABLE_WORTE, [])?;
    conn.execute_batch(worte::CREATE_STR_INDEX_WORTE)?;

    conn.execute(worte_gram_type::CREATE_STR_TABLE_WORTE_TYPE_GRAM, [])?;
    conn.execute_batch(worte_gram_type::CREATE_STR_INDEX_WORTE_TYPE_GRAM)?;

    Ok(())
}
