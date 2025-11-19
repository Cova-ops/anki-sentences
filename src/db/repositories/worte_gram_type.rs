use color_eyre::eyre::Result;
use rusqlite::{Transaction, params};

use crate::db::schemas::worte_gram_type::NewWorteGramTypeSchema;

pub fn bulk_insert_tx(tx: &Transaction, data: &[NewWorteGramTypeSchema]) -> Result<()> {
    let sql = r#"
    INSERT INTO worte_gram_type (id_worte, id_gram_type)
        VALUES (?1, ?2)
        ON CONFLICT DO NOTHING;
    "#;

    let mut stmt = tx.prepare_cached(sql)?;
    for d in data {
        let _ = stmt.query(params![d.id_worte, d.id_gram_type])?;
    }

    Ok(())
}
