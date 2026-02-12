use color_eyre::eyre::Result;

use rusqlite::Connection;

use crate::{
    db::{
        gram_type::GramTypeRepo,
        niveau_liste::NiveauListeRepo,
        schemas::{
            gram_type::EnumGramType, niveau_liste::EnumNiveauListe, wort_gender::EnumWortGender,
        },
        worte_gender::WorteGenderRepo,
    },
    helpers::error_handler::DbError,
};

pub fn init_data(conn: &mut Connection) -> Result<(), DbError> {
    let tx = conn.transaction()?;

    // GenderWorte
    let seed: Vec<_> = EnumWortGender::ALL.iter().map(|d| d.to_new()).collect();
    WorteGenderRepo::bulk_upsert_tx(&tx, &seed)?;

    // NiveauWorte
    let seed: Vec<_> = EnumNiveauListe::ALL.iter().map(|d| d.to_new()).collect();
    NiveauListeRepo::bulk_upsert_tx(&tx, &seed)?;

    // GramType
    let seed: Vec<_> = EnumGramType::ALL.iter().map(|d| d.to_new()).collect();
    GramTypeRepo::bulk_upsert_tx(&tx, &seed)?;

    tx.commit()?;

    Ok(())
}
