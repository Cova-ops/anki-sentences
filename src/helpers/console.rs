use color_eyre::eyre::Result;

use crate::{
    db::{
        self, GeschichlichSetzeRepo,
        schemas::{geschichtlich_setze::NewGeschichtlichSetzeSchema, setze::SetzeSchema},
    },
    helpers::ui,
    utils,
};

const TEXT_SETZE_ONCE: &str = r##"
Para salir pon la palara "exit".
Algunas letras que te pueden ayudar. :)
          - ß ẞ ä ö ü Ä Ö Ü 

  Tema: {thema}
  Oración: {satz}

Por favor traducela...
"##;

/// return:
/// - 0 Finishing sentences
/// - 1 User typed "exit"
pub fn make_setze_exercise(arr: &[SetzeSchema]) -> Result<u8> {
    for s in arr {
        utils::clean_screen();
        let mut s_done = false;

        let db_s = utils::clean_sentences(&s.setze_deutsch);
        for i in 0..2 {
            println!(
                "{}",
                TEXT_SETZE_ONCE
                    .replace("{satz}", &s.setze_spanisch)
                    .replace("{thema}", &s.thema)
            );

            let Some(input) = ui::prompt_nonempty("> ")? else {
                continue;
            };

            if input == "exit" {
                return Ok(1);
            }

            let input = utils::clean_sentences(&input);
            if input == db_s {
                println!("Oración perfecta.");
                s_done = true;

                // Si "i" vale 0, quiere decir que respondio al oración a la primera,
                // se pasa un true
                let new_data = NewGeschichtlichSetzeSchema { setze_id: s.id };
                GeschichlichSetzeRepo::insert_db(&[new_data], i == 0)?;

                break;
            } else {
                println!();
                println!("Oración incorrecta");
            }
        }

        if !s_done {
            println!("La oración correcta es: {}", s.setze_deutsch);
            println!("Schreib es gut, bitte.");

            loop {
                let Some(input) = ui::prompt_nonempty("> ")? else {
                    break;
                };
                if input == "exit" {
                    return Ok(1);
                }

                let input = utils::clean_sentences(&input);
                if input == db_s {
                    let new_data = NewGeschichtlichSetzeSchema { setze_id: s.id };
                    GeschichlichSetzeRepo::insert_db(&[new_data], false)?;
                    break;
                }
            }
        }
    }

    Ok(0)
}

/// return:
/// - 0 Finishing sentences
/// - 1 User typed "exit"
pub fn make_setze_exercise_repeat(arr: &[SetzeSchema]) -> Result<u8> {
    let mut setze_correct: Vec<SetzeSchema> = Vec::from(arr);
    let mut i = 0;
    while !setze_correct.is_empty() {
        let s = setze_correct[i].clone();

        utils::clean_screen();
        let mut s_done = false;
        let db_s = utils::clean_sentences(&s.setze_deutsch);
        for i in 0..2 {
            println!(
                "{}",
                TEXT_SETZE_ONCE
                    .replace("{satz}", &s.setze_spanisch)
                    .replace("{thema}", &s.thema)
            );

            let Some(input) = ui::prompt_nonempty("> ")? else {
                continue;
            };

            if input == "exit" {
                return Ok(1);
            }

            let input = utils::clean_sentences(&input);
            if input == db_s {
                println!("Oración perfecta.");
                s_done = true;

                // Si "i" vale 0, quiere decir que respondio al oración a la primera,
                // se pasa un true
                if i == 0 {
                    let new_data = NewGeschichtlichSetzeSchema { setze_id: s.id };
                    GeschichlichSetzeRepo::insert_db(&[new_data], true)?;
                    setze_correct.remove(i);
                }

                break;
            } else {
                println!();
                println!("Oración incorrecta");
            }
        }

        if !s_done {
            println!("La oración correcta es: {}", s.setze_deutsch);
            println!("Schreib es gut, bitte.");

            loop {
                let Some(input) = ui::prompt_nonempty("> ")? else {
                    break;
                };
                if input == "exit" {
                    return Ok(1);
                }

                let input = utils::clean_sentences(&input);
                if input == db_s {
                    let new_data = NewGeschichtlichSetzeSchema { setze_id: s.id };
                    GeschichlichSetzeRepo::insert_db(&[new_data], false)?;
                    break;
                }
            }
        }

        i += 1;
        i %= arr.len();
    }

    Ok(0)
}
