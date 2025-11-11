use color_eyre::eyre::Result;

use crate::{db::SetzeRepo::fetch_random, helpers::console::make_setze_exercise};

pub fn menu_2_1_random_sentences() -> Result<()> {
    let mut setze = fetch_random(10)?;
    while !setze.is_empty() {
        let r = make_setze_exercise(&setze)?;
        if r == 1 {
            break;
        }

        setze = fetch_random(10)?;
    }

    Ok(())
}
