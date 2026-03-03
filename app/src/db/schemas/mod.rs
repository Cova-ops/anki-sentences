use rusqlite::Connection;

use crate::helpers::error_handler::DbError;

pub mod gram_type;
pub mod niveau_liste;
pub mod setze;
pub mod setze_audio;
pub mod setze_review;
pub mod wort;
pub mod wort_audio;
pub mod wort_gender;
pub mod wort_gram_type;
pub mod wort_review;

pub fn init_schemas(conn: &mut Connection) -> Result<(), DbError> {
    // Active foreign keys
    conn.execute_batch("PRAGMA foreign_keys = ON")?;

    // Gender Worte
    conn.execute_batch(wort_gender::CREATE_STR_TABLE_WORTE_GENDER)?;
    conn.execute_batch(wort_gender::CREATE_STR_INDEX_WORTE_GENDER)?;

    // Niveau Liste
    conn.execute_batch(niveau_liste::CREATE_STR_TABLE_NIVEAU_LISTE)?;
    conn.execute_batch(niveau_liste::CREATE_STR_INDEX_NIVEAU_LISTE)?;

    // Gram Type
    conn.execute_batch(gram_type::CREATE_STR_TABLE_GRAM_TYPE)?;
    conn.execute_batch(gram_type::CREATE_STR_INDEX_GRAM_TYPE)?;

    // Oraciones
    conn.execute_batch(setze::CREATE_STR_TABLE_SETZE)?;
    conn.execute_batch(setze::CREATE_STR_INDEX_SETZE)?;

    conn.execute_batch(setze_review::CREATE_STR_TABLE_SETZE_REVIEW)?;
    conn.execute_batch(setze_review::CREATE_STR_INDEX_SETZE_REVIEW)?;

    conn.execute_batch(setze_audio::CREATE_STR_TABLE_SETZE_AUDIO)?;
    conn.execute_batch(setze_audio::CREATE_STR_INDEX_SETZE_AUDIO)?;

    // Words
    conn.execute_batch(wort::CREATE_STR_TABLE_WORTE)?;
    conn.execute_batch(wort::CREATE_STR_INDEX_WORTE)?;

    conn.execute_batch(wort_gram_type::CREATE_STR_TABLE_WORTE_TYPE_GRAM)?;
    conn.execute_batch(wort_gram_type::CREATE_STR_INDEX_WORTE_TYPE_GRAM)?;

    conn.execute_batch(wort_review::CREATE_STR_TABLE_WORTE_REVIEW)?;
    conn.execute_batch(wort_review::CREATE_STR_INDEX_WORTE_REVIEW)?;

    conn.execute_batch(wort_audio::CREATE_STR_TABLE_WORTE_AUDIO)?;
    conn.execute_batch(wort_audio::CREATE_STR_INDEX_WORTE_AUDIO)?;

    Ok(())
}
