use color_eyre::eyre::Result;
use rand::seq::SliceRandom;
use rusqlite::Connection;

use crate::{
    db::{geschichtlich_setze::GeschichtlichSetzeRepo, setze::SetzeRepo},
    helpers::console::make_setze_exercise,
};

pub fn menu_2_1_random_sentences(conn: &mut Connection) -> Result<()> {
    let mut ids = SetzeRepo::fetch_all_only_ids(conn)?;
    let mut seed_rand = rand::rng();
    ids.shuffle(&mut seed_rand);

    let offset_setze = 10;

    let ids_next: Vec<i32> = ids.drain(..offset_setze).collect();
    let mut setze = SetzeRepo::fetch_by_id(conn, &ids_next)?;

    while !setze.is_empty() {
        let r = make_setze_exercise(&setze)?;
        GeschichtlichSetzeRepo::bulk_insert(conn, &r.1)?;

        if r.0 == 1 {
            break;
        }

        let ids_next: Vec<i32> = ids.drain(..offset_setze).collect();
        setze = SetzeRepo::fetch_by_id(conn, &ids_next)?;
    }

    Ok(())
}
