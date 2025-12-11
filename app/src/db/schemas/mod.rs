use color_eyre::eyre::Result;
use rusqlite::Connection;

pub mod gender_worte;
pub mod geschichtlich_setze;
pub mod gram_type;
pub mod niveau_liste;
pub mod setze;
pub mod worte;
pub mod worte_audio;
pub mod worte_gram_type;
pub mod worte_review;

pub fn init_schemas(conn: &mut Connection) -> Result<()> {
    // Activar las llaves foráneas
    conn.execute("PRAGMA foreign_keys = ON", [])?;

    // Gender Worte
    conn.execute(gender_worte::CREATE_STR_TABLE_GENDER_WORTE, [])?;
    conn.execute_batch(gender_worte::CREATE_STR_INDEX_GENDER_WORTE)?;

    // Niveau Liste
    conn.execute(niveau_liste::CREATE_STR_TABLE_NIVEAU_LISTE, [])?;
    conn.execute_batch(niveau_liste::CREATE_STR_INDEX_NIVEAU_LISTE)?;

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

    conn.execute(worte_review::CREATE_STR_TABLE_WORTE_REVIEW, [])?;
    conn.execute_batch(worte_review::CREATE_STR_INDEX_WORTE_REVIEW)?;

    conn.execute(worte_audio::CREATE_STR_TABLE_WORTE_AUDIO, [])?;
    conn.execute_batch(worte_audio::CREATE_STR_INDEX_WORTE_AUDIO)?;

    Ok(())
}
