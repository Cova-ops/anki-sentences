use color_eyre::eyre::Result;
use rand::seq::SliceRandom;

use crate::{db::SetzeRepo, helpers};

pub fn menu_2_3_schwirig_sentences() -> Result<()> {
    let offset: usize = 10;
    let mut ids_setze: Vec<i32> = SetzeRepo::fetch_id_schwirig_thema(None)?;

    let mut seed_rand = rand::rng();
    ids_setze.shuffle(&mut seed_rand);

    while !ids_setze.is_empty() {
        let aux_ids: Vec<i32> = ids_setze.drain(..offset).collect();

        let setze = SetzeRepo::fetch_by_id(&aux_ids)?;
        let r = helpers::console::make_setze_exercise_repeat(&setze)?;
        if r == 1 {
            return Ok(());
        }
    }
    println!("Oraciones dificiles finalizadas.");

    Ok(())
}
