use color_eyre::eyre::Result;
use once_cell::sync::Lazy;

use crate::db::{SchwirigkeitListeBulkInsert, schemas::schwirigkeit_liste::NewSchwirigkeitSchema};

pub static SEED_SCHWIRIGKEIT_LISTE: Lazy<Vec<NewSchwirigkeitSchema>> = Lazy::new(|| {
    Vec::from([
        NewSchwirigkeitSchema::new(0, "Einfag"),
        NewSchwirigkeitSchema::new(1, "Normal"),
        NewSchwirigkeitSchema::new(2, "Schwirig"),
    ])
});

pub fn init_seeds() -> Result<()> {
    SchwirigkeitListeBulkInsert(&SEED_SCHWIRIGKEIT_LISTE)?;

    // for data in SEED_SCHWIRIGKEIT_LISTE {
    //     conn.execute(
    //         "INSERT INTO schwirigkeit_liste (id, schwirigkeit)
    //     VALUES (?1, ?2)",
    //         data,
    //     )?;
    // }
    Ok(())
}
