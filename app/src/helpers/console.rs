use color_eyre::eyre::Result;

use crate::{
    db::schemas::{geschichtlich_setze::NewGeschichtlichSetzeSchema, setze::SetzeSchema},
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
// TODO: Hacer que esto regrese el arreglo apra meterlo a la DB, que no se conecte el herlper a la
// DB
pub fn make_setze_exercise(arr: &[SetzeSchema]) -> Result<(u8, Vec<NewGeschichtlichSetzeSchema>)> {
    let mut vec_out = Vec::with_capacity(arr.len());
    let mut val_out = 0;

    for s in arr {
        utils::clean_screen();
        let mut s_done = false;

        let db_s = utils::string::clean_sentences(&s.setze_deutsch);
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
                val_out = 1;
                break;
            }

            let input = utils::string::clean_sentences(&input);
            if input == db_s {
                println!("Oración perfecta.");
                s_done = true;

                // Si "i" vale 0, quiere decir que respondio al oración a la primera,
                // se pasa un true
                let new_data = NewGeschichtlichSetzeSchema {
                    setze_id: s.id,
                    result: i == 0,
                };
                vec_out.push(new_data);

                break;
            } else {
                println!();
                println!("Oración incorrecta");
            }
        }

        if !s_done && val_out != 1 {
            println!("La oración correcta es: {}", s.setze_deutsch);
            println!("Schreib es gut, bitte.");

            loop {
                let Some(input) = ui::prompt_nonempty("> ")? else {
                    break;
                };
                if input == "exit" {
                    val_out = 1;
                    break;
                }

                let input = utils::string::clean_sentences(&input);
                if input == db_s {
                    let new_data = NewGeschichtlichSetzeSchema {
                        setze_id: s.id,
                        result: false,
                    };
                    vec_out.push(new_data);
                    break;
                }
            }
        }
    }

    Ok((val_out, vec_out))
}

/// return:
/// - 0 Finishing sentences
/// - 1 User typed "exit"
pub fn make_setze_exercise_repeat(
    arr: &[SetzeSchema],
) -> Result<(u8, Vec<NewGeschichtlichSetzeSchema>)> {
    let mut setze_correct: Vec<SetzeSchema> = Vec::from(arr);
    let mut i = 0;

    let mut vec_out = Vec::with_capacity(arr.len());
    let mut val_out = 0;
    while !setze_correct.is_empty() {
        let s = setze_correct[i].clone();

        utils::clean_screen();
        let mut s_done = false;
        let db_s = utils::string::clean_sentences(&s.setze_deutsch);
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
                val_out = 1;
                break;
            }

            let input = utils::string::clean_sentences(&input);
            if input == db_s {
                println!("Oración perfecta.");
                s_done = true;

                // Si "i" vale 0, quiere decir que respondio al oración a la primera,
                // se pasa un true
                if i == 0 {
                    let new_data = NewGeschichtlichSetzeSchema {
                        setze_id: s.id,
                        result: true,
                    };
                    vec_out.push(new_data);
                    setze_correct.remove(i);
                }

                break;
            } else {
                println!();
                println!("Oración incorrecta");
            }
        }

        if !s_done && val_out != 1 {
            println!("La oración correcta es: {}", s.setze_deutsch);
            println!("Schreib es gut, bitte.");

            loop {
                let Some(input) = ui::prompt_nonempty("> ")? else {
                    break;
                };
                if input == "exit" {
                    val_out = 1;
                    break;
                }

                let input = utils::string::clean_sentences(&input);
                if input == db_s {
                    let new_data = NewGeschichtlichSetzeSchema {
                        setze_id: s.id,
                        result: false,
                    };
                    vec_out.push(new_data);
                    break;
                }
            }
        }

        i += 1;
        i %= arr.len();
    }

    Ok((val_out, vec_out))
}
