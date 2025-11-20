use color_eyre::eyre::Result;
use rusqlite::Connection;

use crate::{
    db::{geschichtlich_setze::GeschichtlichSetzeRepo, setze::SetzeRepo},
    helpers::console::make_setze_exercise,
};

pub fn menu_2_1_random_sentences(conn: &mut Connection) -> Result<()> {
    let mut ids = SetzeRepo::fetch_all_only_ids(conn)?;
    let mut setze = SetzeRepo::fetch_random(conn, 10, &mut ids)?;

    while !setze.is_empty() {
        let r = make_setze_exercise(&setze)?;
        GeschichtlichSetzeRepo::bulk_insert(conn, &r.1)?;

        if r.0 == 1 {
            break;
        }
        setze = SetzeRepo::fetch_random(conn, 10, &mut ids)?;
    }

    Ok(())
}
