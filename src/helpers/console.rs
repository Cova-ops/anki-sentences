use color_eyre::eyre::Result;

use crate::{
    db::{self, SetzeSchema},
    helpers::ui,
    utils,
};

const TEXT_SETZE: &str = r##"
Para salir pon la palara "exit".
Algunas letras que te pueden ayudar. :)
          - ß ẞ ä ö ü Ä Ö Ü 

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
            println!("{}", TEXT_SETZE.replace("{satz}", &s.setze_spanisch));

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
                db::NewGeschichtlichSetzeStruct::new(s.id).insert_db(i == 0)?;

                break;
            } else {
                println!("Oración incorrecta");
                println!("{}", TEXT_SETZE.replace("{satz}", &s.setze_spanisch));
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
                    db::NewGeschichtlichSetzeStruct::new(s.id).insert_db(false)?;
                    break;
                }
            }
        }
    }

    Ok(0)
}
