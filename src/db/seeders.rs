use color_eyre::eyre::Result;
use once_cell::sync::Lazy;

use crate::db::{
    NiveauWorteRepo, SchwirigkeitListeRepo,
    repositories::GenderWorteRepo,
    schemas::{
        gender_worte::{GenderWorteSchema, NewGenderWorteSchema},
        niveau_worte::{NewNiveauWorteSchema, NiveauWorteSchema},
        schwirigkeit_liste::{NewSchwirigkeitListeSchema, SchwirigkeitListeSchema},
    },
};

pub static SEED_SCHWIRIGKEIT_LISTE: Lazy<Vec<NewSchwirigkeitListeSchema>> = Lazy::new(|| {
    Vec::from([
        NewSchwirigkeitListeSchema::new(0, "Einfag"),
        NewSchwirigkeitListeSchema::new(1, "Normal"),
        NewSchwirigkeitListeSchema::new(2, "Schwirig"),
    ])
});

pub static SEED_GENDER_WORTE_LISTE: Lazy<Vec<NewGenderWorteSchema>> = Lazy::new(|| {
    Vec::from([
        NewGenderWorteSchema::new(0, "Maskuline", "der"),
        NewGenderWorteSchema::new(1, "Femenin", "die"),
        NewGenderWorteSchema::new(2, "Neutrum", "das"),
        NewGenderWorteSchema::new(3, "Plural", "die"),
    ])
});

pub static SEED_NIVEAU_LISTE: Lazy<Vec<NewNiveauWorteSchema>> = Lazy::new(|| {
    Vec::from([
        NewNiveauWorteSchema::new(0, "A1"),
        NewNiveauWorteSchema::new(1, "A2"),
        NewNiveauWorteSchema::new(2, "B1"),
        NewNiveauWorteSchema::new(3, "B2"),
        NewNiveauWorteSchema::new(4, "C1"),
        NewNiveauWorteSchema::new(5, "C2"),
    ])
});

pub fn init_data() -> Result<()> {
    let data_sch = SchwirigkeitListeRepo::bulk_insert(&SEED_SCHWIRIGKEIT_LISTE)?;
    let data_gen = GenderWorteRepo::bulk_insert(&SEED_GENDER_WORTE_LISTE)?;
    let data_niv = NiveauWorteRepo::bulk_insert(&SEED_NIVEAU_LISTE)?;

    SchwirigkeitListeSchema::init_data(&data_sch)?;
    GenderWorteSchema::init_data(&data_gen)?;
    NiveauWorteSchema::init_data(&data_niv)?;

    Ok(())
}
