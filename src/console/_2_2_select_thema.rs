use color_eyre::eyre::Result;
use inquire::MultiSelect;

use crate::{db::SetzeRepo, helpers};

pub fn menu_2_2_select_thema() -> Result<()> {
    let titles: Vec<String> = SetzeRepo::fetch_all_titles()?;

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
    let mut setze = SetzeRepo::fetch_where_thema(&ans, offset, limit)?;

    while !setze.is_empty() {
        let r = helpers::console::make_setze_exercise(&setze)?;
        if r == 1 {
            return Ok(());
        }

        offset += 10;
        limit += 10;
        setze = SetzeRepo::fetch_where_thema(&ans, offset, limit)?;
    }
    println!("Oraciones finalizadas del tema.");

    Ok(())
}
