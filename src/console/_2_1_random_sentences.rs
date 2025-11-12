use color_eyre::eyre::Result;

use crate::{db::SetzeRepo, helpers::console::make_setze_exercise};

pub fn menu_2_1_random_sentences() -> Result<()> {
    let mut ids = SetzeRepo::fetch_all_only_ids()?;
    let mut setze = SetzeRepo::fetch_random(10, &mut ids)?;

    while !setze.is_empty() {
        let r = make_setze_exercise(&setze)?;
        if r == 1 {
            break;
        }

        setze = SetzeRepo::fetch_random(10, &mut ids)?;
    }

    Ok(())
}
