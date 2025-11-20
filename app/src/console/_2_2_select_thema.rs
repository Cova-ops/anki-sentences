use color_eyre::eyre::Result;
use inquire::MultiSelect;
use rusqlite::Connection;

use crate::{
    db::{geschichtlich_setze::GeschichtlichSetzeRepo, setze::SetzeRepo},
    helpers,
};

pub fn menu_2_2_select_thema(conn: &mut Connection) -> Result<()> {
    let titles: Vec<String> = SetzeRepo::fetch_all_themas(conn)?;

    let ans = loop {
        let a = MultiSelect::new("Selecciona los temas a repasar:", titles.clone())
            .with_page_size(20)
            .prompt()
            .unwrap();
        if !a.is_empty() {
            break a;
        }
    };

    let mut offset: u32 = 0;
    let mut limit: u32 = 10;
    let mut setze = SetzeRepo::fetch_where_thema(conn, &ans, offset, limit)?;

    while !setze.is_empty() {
        let r = helpers::console::make_setze_exercise(&setze)?;
        GeschichtlichSetzeRepo::bulk_insert(conn, &r.1)?;

        if r.0 == 1 {
            return Ok(());
        }

        setze = SetzeRepo::fetch_where_thema(conn, &ans, offset, limit)?;
        offset += 10;
        limit += 10;
    }
    println!("Oraciones finalizadas del tema.");

    Ok(())
}
