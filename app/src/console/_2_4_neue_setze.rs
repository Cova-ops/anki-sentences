use color_eyre::eyre::Result;
use rand::seq::SliceRandom;
use rusqlite::Connection;

use crate::{
    db::{geschichtlich_setze::GeschichtlichSetzeRepo, setze::SetzeRepo},
    helpers,
};

pub fn menu_2_4_neue_sentences(conn: &mut Connection) -> Result<()> {
    let offset: usize = 10;
    let mut ids_setze: Vec<i32> = SetzeRepo::fetch_id_neue_sentences(conn)?;

    let mut seed_rand = rand::rng();
    ids_setze.shuffle(&mut seed_rand);

    while !ids_setze.is_empty() {
        let aux_ids: Vec<i32> = ids_setze.drain(..offset).collect();
        let setze = SetzeRepo::fetch_by_id(conn, &aux_ids)?;

        let r = helpers::console::make_setze_exercise_repeat(&setze)?;
        GeschichtlichSetzeRepo::bulk_insert(conn, &r.1)?;

        if r.0 == 1 {
            return Ok(());
        }
    }
    println!("Sin mas oraciones nuevas disponibles.");

    Ok(())
}
